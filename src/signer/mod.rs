use crate::backend::SigningBackend;
use crate::protocol::{Request, Response};
use crate::types::BufferError;
use crate::versions::ProtocolVersion;
use log::info;
use nebula::SignerError;
use prost::Message as _;
use std::io::{Read, Write};
use std::marker::PhantomData;

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

    pub fn process_request(
        &self,
        request: Request,
    ) -> Result<
        Response<V::ProposalResponse, V::VoteResponse, V::PubKeyResponse, V::PingResponse>,
        SignerError,
    > {
        let response = match request {
            Request::SignProposal(proposal) => {
                let signable_data = V::proposal_to_bytes(&proposal, &self.chain_id)?;
                let signature = self.signer.sign(&signable_data);
                info!("Signature: {}", hex::encode(&signature));
                info!("Signable data: {}", hex::encode(&signable_data));

                Response::SignedProposal(V::create_proposal_response(
                    Some(proposal.clone()),
                    signature,
                    None,
                ))
            }
            Request::SignVote(vote) => {
                let signable_data = V::vote_to_bytes(&vote, &self.chain_id)?;
                let signature = self.signer.sign(&signable_data);
                info!("Signature: {}", hex::encode(&signature));
                info!("Signable data: {}", hex::encode(&signable_data));
                Response::SignedVote(V::create_vote_response(Some(vote.clone()), signature, None))
            }
            Request::ShowPublicKey => {
                let public_key = self.signer.public_key();
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

                Err(BufferError::NeedMoreBytes) => {
                    // message is not yet over, continue reading
                }
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

#[cfg(test)]
mod tests;
