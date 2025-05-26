use crate::protocol::{Request, Response};
use crate::signer::SigningBackend;
use crate::versions::ProtocolVersion;
use nebula::SignerError;
use prost::Message as _;
use std::io::{Read, Write};
use std::marker::PhantomData;
use std::net::TcpStream;
use std::thread::sleep;
use std::time::Duration;
use tendermint_p2p::secret_connection::SecretConnection;

pub struct TcpSigner<T: SigningBackend, V: ProtocolVersion> {
    signer: T,
    connection: SecretConnection<TcpStream>,
    chain_id: String,
    _version: PhantomData<V>,
}

impl<T: SigningBackend, V: ProtocolVersion> TcpSigner<T, V> {
    pub fn new(signer: T, connection: SecretConnection<TcpStream>, chain_id: String) -> Self {
        Self {
            signer,
            connection,
            chain_id,
            _version: PhantomData,
        }
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        println!("Starting request loop for chain: {}", self.chain_id);

        loop {
            match self.handle_request() {
                Ok(should_continue) => {
                    if !should_continue {
                        println!("Request loop terminated gracefully");
                        break;
                    }
                }
                Err(e) => {
                    eprintln!("Error handling request: {}. Continuing...", e);
                    sleep(Duration::from_millis(100));
                }
            }
        }

        Ok(())
    }

    fn handle_request(&mut self) -> Result<bool, Box<dyn std::error::Error>> {
        let request = self.read_request()?;

        println!("Received request: {:?}", request);

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

        // println!("Sending response: {:?}", response);

        self.send_response(response)?;

        Ok(true)
    }

    fn read_request(&mut self) -> Result<Request, Box<dyn std::error::Error>> {
        let msg_bytes = self.read_complete_message()?;
        let (request, _chain_id) = V::parse_request(msg_bytes)?;
        Ok(request)
    }

    fn read_complete_message(&mut self) -> Result<Vec<u8>, Box<dyn std::error::Error>> {
        let mut msg_bytes: Vec<u8> = vec![];

        loop {
            let mut buf = vec![0; 1024];
            let buf_read = self.connection.read(&mut buf)?;
            buf.truncate(buf_read);
            let chunk_len = buf.len();
            msg_bytes.extend_from_slice(&buf);

            if let Ok(_) = V::Message::decode_length_delimited(msg_bytes.as_ref()) {
                return Ok(msg_bytes);
            }

            if chunk_len < 1024 {
                return Err(Box::new(SignerError::InvalidData));
            }
        }
    }

    fn send_response(
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
        Ok(())
    }
}
