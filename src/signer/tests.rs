use crate::Signer;
use crate::SigningBackend;
use crate::signer::{Request, Response};
use crate::versions::VersionV0_38;
use nebula::proto::v0_38;
use prost::Message;
use std::io::{Cursor, Read, Write};

struct Dummy;

impl SigningBackend for Dummy {
    fn sign(&self, _: &[u8]) -> Vec<u8> {
        vec![0xde, 0xad, 0xbe, 0xef]
    }
    fn public_key(&self) -> Vec<u8> {
        vec![1, 2, 3, 4]
    }
}

type TS = Signer<Dummy, VersionV0_38, Cursor<Vec<u8>>>;

fn create_ping_message() -> Vec<u8> {
    let msg = v0_38::privval::Message {
        sum: Some(v0_38::privval::message::Sum::PingRequest(
            v0_38::privval::PingRequest {},
        )),
    };
    let mut buf = Vec::new();
    msg.encode_length_delimited(&mut buf).unwrap();
    buf
}

#[test]
fn ping_request() {
    let ping_data = create_ping_message();
    let mut s = TS::new(Dummy, Cursor::new(ping_data), "test-chain".into());

    let req = s.read_request().unwrap();
    assert!(matches!(req, Request::PingRequest));

    let resp = s.process_request(req).unwrap();
    assert!(matches!(resp, Response::Ping(_)));

    assert!(s.send_response(resp).is_ok());
}

#[test]
fn multiple_ping_messages_in_stream() {
    let ping1 = create_ping_message();
    let ping2 = create_ping_message();
    let combined = [ping1, ping2].concat();

    let mut s = TS::new(Dummy, Cursor::new(combined), "test-chain".into());

    let req1 = s.read_request().unwrap();
    assert!(matches!(req1, Request::PingRequest));

    let req2 = s.read_request().unwrap();
    assert!(matches!(req2, Request::PingRequest));
}

#[test]
fn varying_chunk_sizes() {
    struct ChunkedReader {
        data: Vec<u8>,
        pos: usize,
        sizes: Vec<usize>,
        chunk_index: usize,
    }

    impl Read for ChunkedReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.pos >= self.data.len() || self.chunk_index >= self.sizes.len() {
                return Ok(0);
            }
            let chunk_size = self.sizes[self.chunk_index];
            let remaining = self.data.len() - self.pos;
            let to_read = chunk_size.min(buf.len()).min(remaining);

            buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
            self.pos += to_read;
            self.chunk_index += 1;
            Ok(to_read)
        }
    }

    impl Write for ChunkedReader {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let ping_data = create_ping_message();
    let reader = ChunkedReader {
        data: ping_data.clone(),
        pos: 0,
        sizes: vec![1, 1, 2, 1],
        chunk_index: 0,
    };

    let mut s = Signer::<Dummy, VersionV0_38, _>::new(Dummy, reader, "test-chain".into());
    let req = s.read_request().unwrap();
    assert!(matches!(req, Request::PingRequest));
}

#[test]
fn fractional_messages_across_reads() {
    let ping1 = create_ping_message();
    let ping2 = create_ping_message();
    let combined = [ping1.clone(), ping2.clone()].concat();

    let ping1_len = ping1.len();
    let ping2_len = ping2.len();

    struct FractionalReader {
        data: Vec<u8>,
        pos: usize,
        read_count: usize,
        ping1_len: usize,
        ping2_len: usize,
    }

    impl Read for FractionalReader {
        fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
            if self.pos >= self.data.len() {
                return Ok(0);
            }

            let chunk_size = match self.read_count {
                0 => self.ping1_len / 2,         // Half of first message
                1 => self.ping1_len / 2 + 1,     // Rest of first + 1 byte of second
                2 => self.ping2_len - 1,         // Most of second message
                _ => self.data.len() - self.pos, // Everything else
            };

            let remaining = self.data.len() - self.pos;
            let to_read = chunk_size.min(buf.len()).min(remaining);

            buf[..to_read].copy_from_slice(&self.data[self.pos..self.pos + to_read]);
            self.pos += to_read;
            self.read_count += 1;
            Ok(to_read)
        }
    }

    impl Write for FractionalReader {
        fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
            Ok(buf.len())
        }
        fn flush(&mut self) -> std::io::Result<()> {
            Ok(())
        }
    }

    let reader = FractionalReader {
        data: combined,
        pos: 0,
        read_count: 0,
        ping1_len,
        ping2_len,
    };

    let mut s = Signer::<Dummy, VersionV0_38, _>::new(Dummy, reader, "test-chain".into());

    let req1 = s.read_request().unwrap();
    assert!(matches!(req1, Request::PingRequest));

    let req2 = s.read_request().unwrap();
    assert!(matches!(req2, Request::PingRequest));
}
