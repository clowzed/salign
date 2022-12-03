use std::{
    fs::OpenOptions,
    io::{BufRead, Write},
};

use extabs::Expand;

use crate::line;

custom_error::custom_error! { pub FileAccessorError
    UnaccessableFile{filename: String, reason: String} = "Failed to get access to {filename}! Reason: {reason}",
    LineWriteError{filename: String, line: String, reason: String} = "Failed to write line: {line} to {filename}! Reason: {reason}",
}

pub struct FileAccessor {}

impl FileAccessor {
    pub fn read(
        filename: &std::path::PathBuf,
        separator: char,
    ) -> Result<Vec<line::Line>, FileAccessorError> {
        let f = match std::fs::File::open(filename) {
            Ok(file) => file,
            Err(e) => {
                return Err(FileAccessorError::UnaccessableFile {
                    filename: filename.to_str().unwrap_or("").into(),
                    reason: e.to_string(),
                })
            }
        };

        let reader: Vec<line::Line> = std::io::BufReader::new(f)
            .lines()
            .into_iter()
            .map(|l| l.unwrap_or_else(|_| "".into()).expandtabs(4))
            .filter_map(|l| line::Line::parse(&l, separator))
            .collect();
        Ok(reader)
    }
    pub fn write(filename: &std::path::Path, lines: Vec<String>) -> Result<(), FileAccessorError> {
        let mut f = match OpenOptions::new().write(true).truncate(true).open(filename) {
            Ok(file) => file,
            Err(e) => {
                return Err(FileAccessorError::UnaccessableFile {
                    filename: filename.to_str().unwrap_or("").into(),
                    reason: e.to_string(),
                });
            }
        };
        for line in lines {
            match f.write((line.clone() + &'\n'.to_string()).as_bytes()) {
                Ok(_) => {}
                Err(e) => {
                    return Err(FileAccessorError::LineWriteError {
                        filename: filename.to_str().unwrap_or("").to_string(),
                        line,
                        reason: e.to_string(),
                    })
                }
            };
        }
        Ok(())
    }
}
