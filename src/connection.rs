use crate::{cluster::SignerRaftNode, error::SignerError};
use ed25519_consensus::SigningKey;
use log::{error, info, warn};
use std::thread::sleep;
use std::time::Duration;
use std::{net::TcpStream, sync::Arc};
use tendermint_p2p::secret_connection::{self, SecretConnection};

pub fn open_secret_connection(
    host: &str,
    port: u16,
    identity_key: SigningKey,
    protocol_version: secret_connection::Version,
    raft_node: &Arc<SignerRaftNode>,
) -> Result<SecretConnection<TcpStream>, SignerError> {
    loop {
        if !raft_node.is_leader() {
            warn!("Not the leader");
            return Err(SignerError::NotLeader(
                ("not leader while opening secret connection").into(),
            ));
        }
        let socket = match TcpStream::connect(format!("{host}:{port}")) {
            Ok(s) => s,
            Err(e) => {
                info!(
                    "Failed to connect to {}:{}: {}. Retrying in 1s...",
                    host, port, e
                );
                sleep(Duration::from_secs(1));
                continue;
            }
        };

        match SecretConnection::new(socket, identity_key.clone(), protocol_version) {
            Ok(conn) => {
                info!("Successfully connected to {}:{}", host, port);
                return Ok(conn);
            }
            Err(error) => {
                error!("SecretConnection failed: {}. Retrying in 1s...", error);
                sleep(Duration::from_secs(1));
                continue;
            }
        }
    }
}
