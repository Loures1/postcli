use crate::postgres::buffer::MutBytes;
use std::io;

pub fn startup_message<'a, I>(params: I, buf: &mut MutBytes) -> io::Result<()>
where
    I: IntoIterator<Item = (&'a str, &'a str)>,
{
    write_body(buf, |buf| {
        buf.put_u32(0x00_03_00_00);
        for (key, value) in params {
            write_cstr(key.as_bytes(), buf)?;
            write_cstr(value.as_bytes(), buf)?;
        }
        buf.put_u8(b"\0");
        Ok(())
    })
}

pub fn query(sql: &str, buf: &mut MutBytes) -> io::Result<()> {
    buf.put_u8(b"Q");
    write_body(buf, |buf| write_cstr(sql.as_bytes(), buf))
}

fn write_body<F, E>(buf: &mut MutBytes, f: F) -> Result<(), E>
where
    F: FnOnce(&mut MutBytes) -> Result<(), E>,
    E: From<io::Error>,
{
    buf.extend_from_slice(&[0; 4]);

    f(buf)?;

    let length: u32 = buf.len() as u32;
    buf.write_u32(0..4, length);

    Ok(())
}

fn write_cstr(s: &[u8], buf: &mut MutBytes) -> Result<(), io::Error> {
    if s.contains(&0) {
        return Err(io::Error::new(
            io::ErrorKind::InvalidInput,
            "string contains embedded null",
        ));
    }

    buf.extend_from_slice(s);
    buf.put_u8(b"\0");

    Ok(())
}
