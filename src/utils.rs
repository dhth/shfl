use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

pub(crate) fn read_from_file(file: &File) -> Result<Vec<String>, std::io::Error> {
    let reader = BufReader::new(file);
    let lines = reader
        .lines()
        .collect::<Result<Vec<String>, std::io::Error>>()?;

    Ok(lines)
}

pub(crate) fn write_to_file(data: Vec<&str>, file_path: &str) -> Result<(), std::io::Error> {
    let mut file = File::options().write(true).truncate(true).open(file_path)?;

    let content = data.join("\n") + "\n";
    file.write(content.as_bytes()).map(|_| ())
}
