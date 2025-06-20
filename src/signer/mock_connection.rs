use std::collections::VecDeque;
use std::io::{self, Read, Write};
use std::sync::mpsc::{Receiver, Sender, TryRecvError};

pub struct MockCometBFTConnection {
    read_buffer: VecDeque<u8>,
    request_receiver: Receiver<Vec<u8>>,
    response_sender: Sender<Vec<u8>>,
}

impl MockCometBFTConnection {
    pub fn new() -> (Self, MockConnectionHandle) {
        let (req_tx, req_rx) = std::sync::mpsc::channel();
        let (res_tx, res_rx) = std::sync::mpsc::channel();

        let mock_conn = Self {
            read_buffer: VecDeque::new(),
            request_receiver: req_rx,
            response_sender: res_tx,
        };

        let handle = MockConnectionHandle {
            request_sender: req_tx,
            response_receiver: res_rx,
        };

        (mock_conn, handle)
    }
}

pub struct MockConnectionHandle {
    pub request_sender: Sender<Vec<u8>>,
    pub response_receiver: Receiver<Vec<u8>>,
}

impl Read for MockCometBFTConnection {
    fn read(&mut self, buf: &mut [u8]) -> io::Result<usize> {
        if self.read_buffer.is_empty() {
            match self.request_receiver.try_recv() {
                Ok(new_request_bytes) => {
                    self.read_buffer.extend(new_request_bytes);
                }
                Err(TryRecvError::Empty) => {
                    return Err(io::Error::new(
                        io::ErrorKind::WouldBlock,
                        "no request available",
                    ));
                }
                Err(TryRecvError::Disconnected) => {
                    return Ok(0);
                }
            }
        }

        let bytes_to_read = std::cmp::min(buf.len(), self.read_buffer.len());
        for i in 0..bytes_to_read {
            buf[i] = self.read_buffer.pop_front().unwrap();
        }

        Ok(bytes_to_read)
    }
}

impl Write for MockCometBFTConnection {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.response_sender
            .send(buf.to_vec())
            .map_err(|e| io::Error::new(io::ErrorKind::BrokenPipe, e.to_string()))?;
        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
