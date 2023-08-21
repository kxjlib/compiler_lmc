use std::{fs::{self, File}, io::Write};

pub fn read_file_to_vec(filename: String) -> std::io::Result<String> {
    fs::read_to_string(filename)
}

pub fn output_to_file(filename: String, contents: String) -> std::io::Result<()> {
    let mut file = File::create(filename.as_str())?;
    file.write_all(contents.as_bytes())?;
    Ok(())
}
