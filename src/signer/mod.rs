use crate::backend::SigningBackend;
use crate::cluster::SignerRaftNode;
use crate::config::Config;
use crate::connection::open_secret_connection;
use crate::error::SignerError;
use crate::protocol::{Request, Response};
use crate::types::{BufferError, SignedMsgType};
use crate::versions::ProtocolVersion;
use log::{debug, info};
use prost::Message as _;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::net::TcpStream;
use std::sync::Arc;
use tendermint_p2p::secret_connection::SecretConnection;

pub struct Signer<T: SigningBackend, V: ProtocolVersion, C: Read + Write> {
    signer: T,
    connection: C,
    chain_id: String,
    _version: PhantomData<V>,
    read_buffer: Vec<u8>,
}

impl<T: SigningBackend, V: ProtocolVersion, C: Read + Write> Signer<T, V, C> {
    pub fn new(signer: T, connection: C, chain_id: String) -> Self {
        Self {
            signer,
            connection,
            chain_id,
            _version: PhantomData,
            read_buffer: Vec::new(),
        }
    }

    // TODO: the signing and sending sig should be split further maybe?
    pub fn process_request(
        &mut self,
        request: Request,
    ) -> Result<
        Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse>,
        SignerError,
    > {
        let response = match request {
            Request::SignProposal(proposal) => {
                let signable_data = V::proposal_to_bytes(&proposal, &self.chain_id)?;
                let signature = self.signer.sign(&signable_data).unwrap();
                debug!("Signature: {}", hex::encode(&signature));
                debug!("Signable data: {}", hex::encode(&signable_data));

                Response::SignedProposal(V::create_proposal_response(
                    Some(proposal.clone()),
                    signature,
                    None,
                ))
            }
            Request::SignVote(vote) => {
                // TODO: chain id should be parsed from the request, and compared to what we're expecting
                // ^ no chain_id in the request. if we configure wrong chain_id in the config
                // ^ actually it IS in the request and it IS in the canonical vote / proposal
                // i just dropped it somewhere
                let signable_data = V::vote_to_bytes(&vote, &self.chain_id)?;
                let signature = self.signer.sign(&signable_data).unwrap();
                // todo: go version also checked for non-zero-length vote extension sign bytes
                // todo: go version also checked for non-nil block id
                let extension_signable_data = V::vote_extension_to_bytes(&vote, &self.chain_id)?;
                if vote.step == SignedMsgType::Precommit
                    && !extension_signable_data.is_empty()
                    && vote
                        .block_id
                        .as_ref()
                        .map(|id| !id.hash.is_empty())
                        .unwrap_or(false)
                {
                    info!("it's a precommit, we will sign the vote ext");
                    let ext_signature = self.signer.sign(&extension_signable_data).unwrap();
                    debug!(
                        "Extension signable data: {}",
                        hex::encode(&extension_signable_data)
                    );
                    debug!("Extension signature: {}", hex::encode(&ext_signature));
                    return Ok(Response::SignedVote(V::create_vote_response(
                        Some(vote.clone()),
                        signature,
                        Some(ext_signature),
                        None,
                    )));
                }
                info!("no vote ext this time");
                debug!("Signature: {}", hex::encode(&signature));
                debug!("Signable data: {}", hex::encode(&signable_data));
                Response::SignedVote(V::create_vote_response(
                    Some(vote.clone()),
                    signature,
                    None,
                    None,
                ))
            }
            Request::ShowPublicKey => {
                let public_key = self.signer.public_key().unwrap();
                Response::PublicKey(V::create_pub_key_response(public_key))
            }
            Request::Ping => Response::Ping(V::create_ping_response()),
        };

        Ok(response)
    }

    pub fn read_request(&mut self) -> Result<Request, SignerError> {
        let msg_bytes = self.read_complete_message()?;
        let (request, _chain_id) = V::parse_request(msg_bytes)?;
        Ok(request)
    }

    // lifetime here is probably not needed. we'd need it if we returned something referencing the buffer
    fn try_read_complete_message(&self, buffer: &[u8]) -> Result<usize, BufferError> {
        match V::Message::decode_length_delimited(buffer) {
            Ok(decoded_message) => {
                let message_len = decoded_message.encoded_len();
                let length_delimiter_len = prost::length_delimiter_len(message_len);
                let total_consumed = length_delimiter_len + message_len;

                Ok(total_consumed)
            }
            Err(_) => Err(BufferError::NeedMoreBytes),
        }
    }

    fn read_complete_message(&mut self) -> Result<Vec<u8>, SignerError> {
        loop {
            match self.try_read_complete_message(&self.read_buffer) {
                Ok(consumed) => {
                    let message = self.read_buffer.drain(..consumed).collect();
                    return Ok(message);
                }

                Err(BufferError::NeedMoreBytes) => {}
            }

            let mut buf = vec![0; 1024];
            let buf_read = self.connection.read(&mut buf)?;

            if buf_read == 0 {
                return Err(SignerError::InvalidData);
            }

            buf.truncate(buf_read);
            self.read_buffer.extend_from_slice(&buf);
        }
    }

    pub fn send_response(
        &mut self,
        response: Response<
            V::ProposalResponse,
            V::VoteResponse,
            V::PubKeyResponse,
            V::PingResponse,
        >,
    ) -> Result<(), SignerError> {
        let response_bytes = V::encode_response(response)?;
        self.connection.write_all(&response_bytes)?;
        self.connection.flush()?;
        Ok(())
    }
}

pub fn create_signer<V: ProtocolVersion>(
    host: &str,
    port: u16,
    identity_key: &ed25519_consensus::SigningKey,
    config: &Config,
    raft_node: &Arc<SignerRaftNode>,
) -> Result<Signer<Box<dyn SigningBackend>, V, SecretConnection<TcpStream>>, SignerError> {
    info!("Connecting to CometBFT at {}:{}", host, port);

    let conn = open_secret_connection(
        host,
        port,
        identity_key.clone(),
        tendermint_p2p::secret_connection::Version::V0_34,
        raft_node,
    )?;

    let backend = crate::backend::create_backend(config)?;

    Ok(Signer::new(backend, conn, config.chain_id.clone()))
}

#[cfg(test)]
pub mod mock_connection;

#[cfg(test)]
mod tests;
