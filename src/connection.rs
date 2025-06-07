use ed25519_consensus::SigningKey;
use log::{error, info};
use nebula::SignerError;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use tendermint_p2p::secret_connection::{self, SecretConnection};

pub fn open_secret_connection(
    host: &str,
    port: u16,
    identity_key: SigningKey,
    protocol_version: secret_connection::Version,
) -> Result<SecretConnection<TcpStream>, SignerError> {
    loop {
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
