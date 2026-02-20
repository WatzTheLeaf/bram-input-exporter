use crate::save::save;

mod save;

pub fn export_inputs(path: &str, raw_inputs: Vec<u8>, rate: u8) -> std::io::Result<()> {
    save(path.as_ref(), &raw_inputs, rate)
}