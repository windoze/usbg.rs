use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn write_data<P: AsRef<Path>>(output_path: P, data: &[u8]) -> io::Result<()> {
    let mut f = fs::File::create(output_path)?;
    f.write_all(data)?;

    Ok(())
}

pub fn create_dir_if_not_exists<P: AsRef<Path>>(path: P) -> io::Result<()> {
    if !path.as_ref().exists() {
        fs::create_dir_all(path)?;
    }

    Ok(())
}
