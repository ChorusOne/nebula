use std::{
    net::TcpStream,
    sync::{Arc, Mutex},
    thread,
    time::Duration,
};

use log::{error, info, warn};
use tendermint_p2p::secret_connection::SecretConnection;

use crate::{
    backend::SigningBackend,
    cluster::SignerRaftNode,
    config::Config,
    error::SignerError,
    handler::SigningHandler,
    signer::{self, Signer},
    versions::ProtocolVersion,
};

pub fn wait_for_leader(raft_node: &Arc<SignerRaftNode>) {
    while raft_node.leader_id().is_none() {
        thread::sleep(Duration::from_millis(200));
    }
    info!("Current leader: {}", raft_node.leader_id().unwrap());
}

pub fn wait_as_follower(raft_node: &Arc<SignerRaftNode>) {
    info!("This node is a follower, standing by…");
    while !raft_node.is_leader() {
        thread::sleep(Duration::from_secs(1));
        if let Some(leader) = raft_node.leader_id() {
            info!("Leader is: {}", leader);
        }
    }
}

pub fn run_leader<V: ProtocolVersion + Send + 'static>(
    config: &Config,
    raft_node: &Arc<SignerRaftNode>,
) -> Result<(), SignerError> {
    info!(
        "Running leader loop for {} connections",
        config.connections.len()
    );

    let config = Arc::new(config.clone());
    let signing_lock = Arc::new(Mutex::new(())); // TODO: a lot of stuff depends on this single lock

    let handles: Vec<_> = config
        .connections
        .iter()
        .map(|conn| {
            let raft_node = Arc::clone(raft_node);
            let config = Arc::clone(&config);
            let signing_lock = Arc::clone(&signing_lock);
            let host = conn.host.clone();
            let port = conn.port;

            thread::spawn(move || {
                handle_connection::<V>(host, port, config, raft_node, signing_lock)
            })
        })
        .collect();

    for handle in handles {
        if let Err(e) = handle.join().expect("Handler thread panicked") {
            error!("Connection handler error: {}", e);
        }
    }

    Ok(())
}

fn handle_connection<V: ProtocolVersion + Send + 'static>(
    host: String,
    port: u16,
    config: Arc<Config>,
    raft_node: Arc<SignerRaftNode>,
    signing_lock: Arc<Mutex<()>>,
) -> Result<(), SignerError> {
    let mut retry_count = 0;
    let identity_key = ed25519_consensus::SigningKey::new(rand_core::OsRng);

    let mut signer = signer::create_signer::<V>(&host, port, &identity_key, &config, &raft_node)?;

    loop {
        if !raft_node.is_leader() {
            warn!("Leadership lost for {}:{}", host, port);
            break;
        }

        match SigningHandler::<V>::handle_single_request(&mut signer, &raft_node, &signing_lock) {
            Ok(()) => {
                retry_count = 0;
            }
            Err(e) => {
                if !raft_node.is_leader() {
                    break;
                }

                error!("Error handling request from {}:{} - {}", host, port, e);

                match reconnect::<V>(
                    &host,
                    port,
                    &identity_key,
                    &config,
                    &raft_node,
                    &mut retry_count,
                ) {
                    Ok(new_signer) => signer = new_signer,
                    Err(_) => continue,
                }
            }
        }
    }

    Ok(())
}

fn reconnect<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
    raft_node: &Arc<SignerRaftNode>,
    retry_count: &mut u32,
) -> Result<Signer<Box<dyn SigningBackend>, V, SecretConnection<TcpStream>>, SignerError> {
    const MAX_RETRY_DELAY: Duration = Duration::from_secs(30);

    loop {
        if !raft_node.is_leader() {
            return Err(SignerError::Other("Leadership lost".into()));
        }

        *retry_count += 1;
        let delay =
            Duration::from_millis(100 * 2_u64.pow((*retry_count).min(10))).min(MAX_RETRY_DELAY);

        warn!(
            "Reconnection attempt {} for {}:{} in {:?}",
            retry_count, host, port, delay
        );
        thread::sleep(delay);

        match signer::create_signer::<V>(host, port, identity_key, config, &raft_node) {
            Ok(signer) => {
                info!("Successfully reconnected to {}:{}", host, port);
                *retry_count = 0;
                return Ok(signer);
            }
            Err(e) => {
                error!("Reconnection failed for {}:{} - {}", host, port, e);
            }
        }
    }
}
