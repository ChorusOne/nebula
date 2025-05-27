use crate::backend::SigningBackend;
use crate::protocol::{Request, Response};
use crate::versions::ProtocolVersion;
use nebula::SignerError;
use prost::Message as _;
use std::io::{Read, Write};
use std::marker::PhantomData;

#[derive(Debug)]
pub enum InsufficientData {
    NeedMoreBytes,
}

impl std::fmt::Display for InsufficientData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Need more data to complete message")
    }
}

impl std::error::Error for InsufficientData {}
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
        Response<
            V::SignedProposalResponse,
            V::SignedVoteResponse,
            V::PubKeyResponse,
            V::PingResponse,
        >,
        Box<dyn std::error::Error>,
    > {
        let response = match request {
            Request::SignProposal(proposal) => {
                let signable_data = V::proposal_to_bytes(&proposal, &self.chain_id)?;
                let signature = self.signer.sign(&signable_data);
                Response::SignedProposal(V::create_signed_proposal_response(signature, None))
            }
            Request::SignVote(vote) => {
                let signable_data = V::vote_to_bytes(&vote, &self.chain_id)?;
                let signature = self.signer.sign(&signable_data);
                Response::SignedVote(V::create_signed_vote_response(signature, None))
            }
            Request::ShowPublicKey => {
                let public_key = self.signer.public_key();
                Response::PublicKey(V::create_pub_key_response(public_key))
            }
            Request::PingRequest => Response::Ping(V::create_ping_response()),
        };

        Ok(response)
    }

    pub fn read_request(&mut self) -> Result<Request, Box<dyn std::error::Error>> {
        let msg_bytes = self.read_complete_message()?;
        let (request, _chain_id) = V::parse_request(msg_bytes)?;
        Ok(request)
    }

    // lifetime here is probably not needed. we'd need it if we returned something referencing the buffer
    fn try_read_complete_message(&self, buffer: &[u8]) -> Result<usize, InsufficientData> {
        match V::Message::decode_length_delimited(buffer) {
            Ok(decoded_message) => {
                let message_len = decoded_message.encoded_len();
                let length_delimiter_len = prost::length_delimiter_len(message_len);
                let total_consumed = length_delimiter_len + message_len;

                Ok(total_consumed)
            }
            Err(_) => Err(InsufficientData::NeedMoreBytes),
        }
    }

    fn read_complete_message(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        loop {
            match self.try_read_complete_message(&self.read_buffer) {
                Ok(consumed) => {
                    let message = self.read_buffer.drain(..consumed).collect();
                    return Ok(message);
                }

                Err(InsufficientData::NeedMoreBytes) => {
                    // message is not yet over, continue reading
                }
            }

            let mut buf = vec![0; 1024];
            let buf_read = self.connection.read(&mut buf)?;

            if buf_read == 0 {
                return Err(Box::new(SignerError::InvalidData));
            }

            buf.truncate(buf_read);
            self.read_buffer.extend_from_slice(&buf);
        }
    }

    pub fn send_response(
        &mut self,
        response: Response<
            V::SignedProposalResponse,
            V::SignedVoteResponse,
            V::PubKeyResponse,
            V::PingResponse,
        >,
    ) -> Result<(), Box<dyn std::error::Error>> {
        let response_bytes = V::encode_response(response)?;
        self.connection.write_all(&response_bytes)?;
        self.connection.flush()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests;
