use std::io::Write;
use tempfile::NamedTempFile;

pub fn bytes_to_temp_file(bytes: &[u8]) -> std::io::Result<NamedTempFile> {
    let mut file = NamedTempFile::new()?;
    file.write_all(bytes)?;
    file.flush()?;
    Ok(file)
}
