use std::io::Read;

pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}

impl Read for Bytes {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize, std::io::Error> {
        self.0.extend_from_slice(buf);
        Ok(buf.len())
    }
}

pub struct MutBytes(Vec<u8>);

impl MutBytes {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn extend_from_slice(&mut self, other: &[u8]) {
        self.0.extend_from_slice(other);
    }

    pub fn put_u8(&mut self, u8: &[u8]) {
        self.extend_from_slice(u8);
    }

    pub fn put_u32(&mut self, u32: u32) {
        self.extend_from_slice(&u32.to_be_bytes());
    }

    pub fn write_u32(&mut self, slice: std::ops::Range<usize>, u32: u32) {
        let u32: &[u8; 4] = &u32.to_be_bytes();

        slice
            .enumerate()
            .for_each(|(u32_p, buff_p)| self.0[buff_p] = u32[u32_p]);
    }

    pub fn len(&self) -> usize {
        self.0.len()
    }

    pub fn as_slice(&self) -> &[u8] {
        &self.0
    }
}
