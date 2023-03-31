use bytes::{BytesMut, BufMut, Bytes};
use std::io;

pub struct BytesMutWriteTransport {
    bytes: BytesMut,
}

impl BytesMutWriteTransport {
    pub fn new() -> Self {
        Self {
            bytes: BytesMut::new(),
        }
    }

    pub fn into_bytes(self) -> Bytes {
        self.bytes.freeze()
    }
}

impl io::Write for BytesMutWriteTransport {
    fn write(&mut self, buf: &[u8]) -> io::Result<usize> {
        self.bytes.put(buf);

        Ok(buf.len())
    }

    fn flush(&mut self) -> io::Result<()> {
        Ok(())
    }
}
