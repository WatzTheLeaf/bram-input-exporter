use std::fs::{File};
use std::io::{BufWriter, Write};
use std::path::Path;
use byteorder::{LittleEndian, WriteBytesExt};

pub(crate) fn save(path: &Path, data: &[u8], rate: u8) -> std::io::Result<()> {
    let mut file = BufWriter::new(File::create(path)?);
    file.write_all(b"BRAM-INPUT-DATA")?;
    file.write_u8(rate)?;
    file.write_u64::<LittleEndian>(data.len() as u64)?;
    file.write_all(data)?;
    Ok(())
}