use std::io;
use std::ops::{Range, RangeFrom};

pub struct Bytes(Vec<u8>);

impl Bytes {
    pub fn new(buf: &[u8]) -> Self {
        Self(Vec::from(buf))
    }

    pub fn get(&self, i: usize) -> &u8 {
        &self.0[i]
    }

    pub fn slice(&self, r: &[RangeFrom<usize>; 1]) -> &[u8] {
        &self.0[r.first().unwrap().clone()]
    }

    pub fn read_u32(&self, r: &[Range<usize>; 1]) -> u32 {
        let r: Range<usize> = r.first().unwrap().clone();

        let mut u32b: [u8; 4] = [0; 4];

        self.range(r, |bytes| {
            if bytes.len() != 4 {
                return Err(io::Error::new(
                    io::ErrorKind::InvalidInput,
                    "The range between the start and the end should to be 4",
                ));
            };

            bytes
                .iter()
                .take(4)
                .enumerate()
                .for_each(|(key, value)| u32b[key] = *value);

            Ok(())
        })
        .unwrap();

        u32::from_be_bytes(u32b)
    }

    fn range<F, E>(&self, r: Range<usize>, f: F) -> Result<(), E>
    where
        F: FnOnce(&[u8]) -> Result<(), E>,
        E: From<io::Error>,
    {
        f(&self.0[r])
    }

    pub fn len(&self) -> usize {
        self.0.len()
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

#[cfg(test)]
mod test_buff {
    use super::Bytes;

    #[test]
    fn read_u32_is_works() {
        let buf: Bytes = Bytes::new(&0u32.to_be_bytes());

        let num = buf.read_u32(&[0..4]);
        let expected_num = 0u32;

        assert_eq!(num, expected_num);
    }

    #[test]
    #[should_panic(expected = "The range between the start and the end should to be 4")]
    fn read_u32_range_cant_to_be_graeter_4() {
        let buf: Bytes = Bytes::new(&0u64.to_be_bytes());

        buf.read_u32(&[0..5]);
    }
}
