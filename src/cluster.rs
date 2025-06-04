use std::collections::{HashMap, HashSet};
use std::fs;
use std::io::{ErrorKind, Read, Write};
use std::net::{SocketAddr, TcpListener, TcpStream};
use std::path::Path;
use std::sync::{Arc, Mutex, RwLock};
use std::thread;
use std::time::{Duration, Instant};

use log::{error, info, warn};
use serde::{Deserialize, Serialize};

#[derive(Debug)]
enum Message {
    Election(u64),
    Coordinator(u64),
    ReplicateState(Vec<u8>),
    Ack,
    Heartbeat,
}

impl Message {
    fn to_bytes(&self) -> Vec<u8> {
        match self {
            Message::Election(id) => {
                let mut buf = vec![0x01];
                buf.extend(&id.to_be_bytes());
                buf
            }
            Message::Coordinator(id) => {
                let mut buf = vec![0x02];
                buf.extend(&id.to_be_bytes());
                buf
            }
            Message::ReplicateState(data) => {
                let mut buf = vec![0x03];
                let len = data.len() as u32;
                buf.extend(&len.to_be_bytes());
                buf.extend(data);
                buf
            }
            Message::Ack => vec![0x04],
            Message::Heartbeat => vec![0x05],
        }
    }

    fn read_from(stream: &mut TcpStream) -> std::io::Result<Message> {
        let mut tag_buf = [0u8; 1];
        stream.read_exact(&mut tag_buf)?;

        match tag_buf[0] {
            0x01 => {
                let mut id_buf = [0u8; 8];
                stream.read_exact(&mut id_buf)?;
                Ok(Message::Election(u64::from_be_bytes(id_buf)))
            }
            0x02 => {
                let mut id_buf = [0u8; 8];
                stream.read_exact(&mut id_buf)?;
                Ok(Message::Coordinator(u64::from_be_bytes(id_buf)))
            }
            0x03 => {
                let mut len_buf = [0u8; 4];
                stream.read_exact(&mut len_buf)?;
                let len = u32::from_be_bytes(len_buf) as usize;
                let mut data = vec![0u8; len];
                stream.read_exact(&mut data)?;
                Ok(Message::ReplicateState(data))
            }
            0x04 => Ok(Message::Ack),
            0x05 => Ok(Message::Heartbeat),
            other => Err(std::io::Error::new(
                ErrorKind::InvalidData,
                format!("Unknown message tag: {:#x}", other),
            )),
        }
    }

    fn write_to(&self, stream: &mut TcpStream) -> std::io::Result<()> {
        stream.write_all(&self.to_bytes())
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct ConsensusData {
    pub height: i64,
    pub round: i64,
    pub step: u8,
}

impl std::fmt::Display for ConsensusData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "ConsensusData {{ height: {}, round: {}, step: {} }}",
            self.height, self.round, self.step
        )
    }
}

impl ConsensusData {
    pub fn persist_to_file(&self, path: &Path) -> std::io::Result<()> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(path, json)
    }

    pub fn load_from_file(path: &Path) -> Option<ConsensusData> {
        let json = fs::read_to_string(path).ok()?;
        serde_json::from_str(&json).ok()
    }
    fn to_bytes(&self) -> Vec<u8> {
        let mut b = Vec::with_capacity(17);
        b.extend(&self.height.to_be_bytes());
        b.extend(&self.round.to_be_bytes());
        b.push(self.step);
        b
    }

    fn from_bytes(buf: &[u8]) -> Option<ConsensusData> {
        if buf.len() != 17 {
            return None;
        }
        let height = i64::from_be_bytes(buf[0..8].try_into().unwrap());
        let round = i64::from_be_bytes(buf[8..16].try_into().unwrap());
        let step = buf[16];
        Some(ConsensusData {
            height,
            round,
            step,
        })
    }
}

pub struct Cluster {
    pub node_id: u64,
    pub peers: Vec<SocketAddr>,
    pub _cluster_port: u16,
    pub state_file: String,

    connections: Arc<Mutex<HashMap<u64, TcpStream>>>,
    current_leader: Arc<RwLock<Option<u64>>>,
    is_leader: Arc<RwLock<bool>>,

    pending_acks: Arc<Mutex<HashSet<u64>>>,
    ack_count: Arc<Mutex<usize>>,
}

impl Cluster {
    pub fn new(
        node_id: u64,
        peers: Vec<SocketAddr>,
        cluster_port: u16,
        state_file: String,
    ) -> std::io::Result<Arc<Cluster>> {
        let connections = Arc::new(Mutex::new(HashMap::new()));
        let current_leader = Arc::new(RwLock::new(None));
        let is_leader = Arc::new(RwLock::new(false));
        let pending_acks = Arc::new(Mutex::new(HashSet::new()));
        let ack_count = Arc::new(Mutex::new(0));

        let cluster = Arc::new(Cluster {
            node_id,
            peers: peers.clone(),
            _cluster_port: cluster_port,
            state_file: state_file.clone(),
            connections: connections.clone(),
            current_leader: current_leader.clone(),
            is_leader: is_leader.clone(),
            pending_acks: pending_acks.clone(),
            ack_count: ack_count.clone(),
        });

        Self::start_listener(
            cluster_port,
            node_id,
            connections.clone(),
            current_leader.clone(),
            is_leader.clone(),
            state_file.clone(),
            pending_acks.clone(),
            ack_count.clone(),
        );

        Self::start_dialer(
            peers,
            node_id,
            connections.clone(),
            current_leader.clone(),
            is_leader.clone(),
            state_file,
            pending_acks.clone(),
            ack_count.clone(),
        );

        Self::start_election_coordinator(cluster.clone());

        Ok(cluster)
    }

    fn start_listener(
        port: u16,
        node_id: u64,
        connections: Arc<Mutex<HashMap<u64, TcpStream>>>,
        current_leader: Arc<RwLock<Option<u64>>>,
        is_leader: Arc<RwLock<bool>>,
        state_file: String,
        pending_acks: Arc<Mutex<HashSet<u64>>>,
        ack_count: Arc<Mutex<usize>>,
    ) {
        thread::spawn(move || {
            let listener =
                TcpListener::bind(("0.0.0.0", port)).expect("Failed to bind cluster port");

            for stream_res in listener.incoming() {
                match stream_res {
                    Ok(mut stream) => {
                        let mut id_buf = [0u8; 8];
                        if let Err(e) = stream.read_exact(&mut id_buf) {
                            error!("Error reading peer_id: {}", e);
                            continue;
                        }
                        let peer_id = u64::from_be_bytes(id_buf);

                        if let Err(e) = stream.write_all(&node_id.to_be_bytes()) {
                            error!("Error sending node_id to peer {}: {}", peer_id, e);
                            continue;
                        }

                        connections
                            .lock()
                            .unwrap()
                            .insert(peer_id, stream.try_clone().unwrap());

                        Self::spawn_message_handler(
                            stream,
                            peer_id,
                            node_id,
                            connections.clone(),
                            current_leader.clone(),
                            is_leader.clone(),
                            state_file.clone(),
                            pending_acks.clone(),
                            ack_count.clone(),
                        );
                    }
                    Err(e) => error!("Error accepting connection: {}", e),
                }
            }
        });
    }

    fn start_dialer(
        peers: Vec<SocketAddr>,
        node_id: u64,
        connections: Arc<Mutex<HashMap<u64, TcpStream>>>,
        current_leader: Arc<RwLock<Option<u64>>>,
        is_leader: Arc<RwLock<bool>>,
        state_file: String,
        pending_acks: Arc<Mutex<HashSet<u64>>>,
        ack_count: Arc<Mutex<usize>>,
    ) {
        thread::spawn(move || {
            for peer_addr in peers {
                match TcpStream::connect_timeout(&peer_addr, Duration::from_secs(2)) {
                    Ok(mut stream) => {
                        if let Err(e) = stream.write_all(&node_id.to_be_bytes()) {
                            error!("Error writing node_id to {}: {}", peer_addr, e);
                            continue;
                        }

                        let mut id_buf = [0u8; 8];
                        if let Err(e) = stream.read_exact(&mut id_buf) {
                            error!("Error reading peer_id from {}: {}", peer_addr, e);
                            continue;
                        }
                        let peer_id = u64::from_be_bytes(id_buf);

                        connections
                            .lock()
                            .unwrap()
                            .insert(peer_id, stream.try_clone().unwrap());

                        Self::spawn_message_handler(
                            stream,
                            peer_id,
                            node_id,
                            connections.clone(),
                            current_leader.clone(),
                            is_leader.clone(),
                            state_file.clone(),
                            pending_acks.clone(),
                            ack_count.clone(),
                        );
                    }
                    Err(e) => error!("Could not connect to {}: {}", peer_addr, e),
                }
            }
        });
    }

    fn spawn_message_handler(
        mut stream: TcpStream,
        peer_id: u64,
        node_id: u64,
        connections: Arc<Mutex<HashMap<u64, TcpStream>>>,
        current_leader: Arc<RwLock<Option<u64>>>,
        is_leader: Arc<RwLock<bool>>,
        state_file: String,
        pending_acks: Arc<Mutex<HashSet<u64>>>,
        ack_count: Arc<Mutex<usize>>,
    ) {
        thread::spawn(move || {
            loop {
                match Message::read_from(&mut stream) {
                    Ok(Message::Coordinator(new_leader_id)) => {
                        *current_leader.write().unwrap() = Some(new_leader_id);
                        *is_leader.write().unwrap() = new_leader_id == node_id;
                    }
                    Ok(Message::ReplicateState(data)) => {
                        if let Some(cd) = ConsensusData::from_bytes(&data) {
                            let path = Path::new(&state_file);
                            if let Err(e) = cd.persist_to_file(path) {
                                error!("Failed to persist state: {}", e);
                            }
                            let _ = Message::Ack.write_to(&mut stream);
                        } else {
                            error!("Received malformed state data");
                        }
                    }
                    Ok(Message::Ack) => {
                        let mut pending = pending_acks.lock().unwrap();
                        if pending.remove(&peer_id) {
                            *ack_count.lock().unwrap() += 1;
                        }
                    }
                    Ok(Message::Heartbeat) => {}
                    Ok(Message::Election(_)) => {}
                    Err(ref e) if e.kind() == ErrorKind::UnexpectedEof => {
                        break;
                    }
                    Err(e) => {
                        error!("Stream read error: {}", e);
                        break;
                    }
                }
            }

            connections.lock().unwrap().remove(&peer_id);
        });
    }

    fn start_election_coordinator(cluster: Arc<Cluster>) {
        thread::spawn(move || {
            cluster.run_election();

            cluster.watch_leader();
        });
    }

    fn run_election(&self) {
        let mut candidates = vec![self.node_id];

        let total_nodes = self.peers.len() + 1;
        let majority = (total_nodes / 2) + 1;
        loop {
            let online = self.connections.lock().unwrap().len() + 1; // note: adding +1 everywhere is stupid FIX IT
            if online >= majority {
                break;
            }
            warn!("no majority reached, connections: {}", online);
            thread::sleep(Duration::from_millis(200));
        }
        {
            let conns = self.connections.lock().unwrap();
            candidates.extend(conns.keys().copied());
        }
        info!("running election, candidates: {:?}", candidates);
        // get bullied
        let new_leader = *candidates.iter().max().unwrap();

        *self.current_leader.write().unwrap() = Some(new_leader);
        *self.is_leader.write().unwrap() = new_leader == self.node_id;

        // show em whos the boss
        let conns = self.connections.lock().unwrap();
        for (_, stream) in conns.iter() {
            if let Ok(mut s) = stream.try_clone() {
                let _ = Message::Coordinator(new_leader).write_to(&mut s);
            }
        }
    }

    fn watch_leader(&self) {
        loop {
            thread::sleep(Duration::from_secs(2));

            let leader = *self.current_leader.read().unwrap();
            if let Some(leader_id) = leader {
                if leader_id == self.node_id {
                    continue;
                }

                let needs_reelection = {
                    let conns = self.connections.lock().unwrap();
                    match conns.get(&leader_id) {
                        Some(stream) => {
                            if let Ok(mut s) = stream.try_clone() {
                                Message::Heartbeat.write_to(&mut s).is_err()
                            } else {
                                true
                            }
                        }
                        None => true,
                    }
                };

                if needs_reelection {
                    error!("Leader {} is down, triggering re-election", leader_id);
                    *self.current_leader.write().unwrap() = None;
                    *self.is_leader.write().unwrap() = false;
                    self.run_election();
                }
            }
        }
    }

    // todo: reinitiate connetions to peers
    pub fn replicate_state(&self, new_state: ConsensusData) -> std::io::Result<()> {
        if !*self.is_leader.read().unwrap() {
            return Err(std::io::Error::new(
                ErrorKind::Other,
                "Not leader: cannot replicate state",
            ));
        }

        let path = Path::new(&self.state_file);
        new_state.persist_to_file(path)?;

        let serialized = new_state.to_bytes();
        let total_nodes: usize;

        {
            let mut pending = self.pending_acks.lock().unwrap();
            pending.clear();

            let conns = self.connections.lock().unwrap();
            total_nodes = conns.len() + 1; // todo: now including self?

            for (&peer_id, stream) in conns.iter() {
                pending.insert(peer_id);
                if let Ok(mut s) = stream.try_clone() {
                    let _ = Message::ReplicateState(serialized.clone()).write_to(&mut s);
                }
            }
        }

        *self.ack_count.lock().unwrap() = 1;

        let majority = (total_nodes / 2) + 1;
        let deadline = Instant::now() + Duration::from_secs(3);

        loop {
            let acks = *self.ack_count.lock().unwrap();
            if acks >= majority {
                return Ok(());
            }

            if Instant::now() > deadline {
                return Err(std::io::Error::new(
                    ErrorKind::TimedOut,
                    format!("Only {} ACKs received, need {}", acks, majority),
                ));
            }

            // todo: how to wait for more acks
            thread::sleep(Duration::from_millis(50));
        }
    }

    pub fn is_leader(&self) -> bool {
        *self.is_leader.read().unwrap()
    }

    pub fn leader_id(&self) -> Option<u64> {
        *self.current_leader.read().unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::path::PathBuf;

    fn make_addr(port: u16) -> SocketAddr {
        format!("127.0.0.1:{}", port).parse().unwrap()
    }

    #[test]
    fn test_election_and_replication() {
        let base = std::env::temp_dir();
        let ports = [60001u16, 60002, 60003];
        let ids = [1u64, 2, 3];

        let peers_addrs: Vec<_> = ports.iter().map(|&p| make_addr(p)).collect();

        let mut state_files = Vec::new();
        for &id in &ids {
            let mut path = PathBuf::from(&base);
            path.push(format!("cluster_test_node_{}_state.bin", id));
            let _ = fs::remove_file(&path);
            state_files.push(path);
        }

        let mut clusters = Vec::new();
        for i in 0..3 {
            let c = Cluster::new(
                ids[i],
                peers_addrs.clone(),
                ports[i],
                state_files[i].to_str().unwrap().to_string(),
            )
            .expect("Failed to start cluster");
            clusters.push(c);
        }

        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let leaders: Vec<_> = clusters.iter().filter_map(|c| c.leader_id()).collect();

            if leaders.len() == 3 && leaders.iter().all(|&l| l == leaders[0]) {
                break;
            }

            if Instant::now() > deadline {
                panic!("Timeout: cluster did not elect a common leader");
            }
            thread::sleep(Duration::from_millis(50));
        }

        for c in &clusters {
            assert_eq!(c.leader_id(), Some(3));
        }

        let leader = clusters
            .iter()
            .find(|c| c.is_leader())
            .expect("No leader found");

        let new_state = ConsensusData {
            height: 42,
            round: 7,
            step: 2,
        };

        leader
            .replicate_state(new_state)
            .expect("Failed to replicate state");

        thread::sleep(Duration::from_millis(500));

        for path in &state_files {
            let cd = ConsensusData::load_from_file(&path).expect("Invalid state");
            assert_eq!(cd.height, 42);
            assert_eq!(cd.round, 7);
            assert_eq!(cd.step, 2);
        }

        for path in state_files {
            let _ = fs::remove_file(path);
        }
    }
}
