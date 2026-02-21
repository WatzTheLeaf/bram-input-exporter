use std::io::Error;
use std::io::ErrorKind::InvalidInput;
use std::path::Path;
use crate::save::save;

mod save;

pub fn export_inputs(path: &str, raw_inputs: &[u8], rate: u8) -> std::io::Result<()> {
    let path = Path::new(path);
    if path.extension().map_or(true, |ext| ext != "dat") {
        return Err(Error::new(InvalidInput, "Invalid file format given"))
    }
    save(path.as_ref(), &raw_inputs, rate)
}