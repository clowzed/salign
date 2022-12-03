use std::{fs::File, path::PathBuf};

custom_error::custom_error! {pub FileManagerError
    UnaccessableFile{file: String, reason: String} = "Failed to access file: {file}! Reason: {reason}",
    UnaccessableDirectory{reason: String} = "Failed to access directory! Reason : {reason}",
    CleanError{reason: String} = "Failed to clean directory! Reason: {reason}"
}

pub struct FileManager {
    temporary_directory: std::path::PathBuf,
}

impl FileManager {
    pub fn new(temporary_directory: std::path::PathBuf) -> Self {
        let s = Self {
            temporary_directory,
        };
        s.create_temporary_directory().ok();
        s
    }

    pub fn create_temporary_file(&self) -> Result<std::path::PathBuf, FileManagerError> {
        self.create_temporary_directory().ok();

        let file = self
            .temporary_directory
            .join(format!("{}", uuid::Uuid::new_v4()));

        match File::create(&file) {
            Ok(_) => Ok(file),
            Err(e) => Err(FileManagerError::UnaccessableFile {
                file: file.to_str().unwrap_or("").into(),
                reason: e.to_string(),
            }),
        }
    }

    pub fn create_temporary_directory(&self) -> Result<(), FileManagerError> {
        match self.temporary_directory.exists() {
            true => Ok(()),
            false => match std::fs::create_dir(&self.temporary_directory) {
                Ok(()) => Ok(()),
                Err(e) => Err(FileManagerError::UnaccessableDirectory {
                    reason: (e.to_string()),
                }),
            },
        }
    }

    pub fn remove_temporary_directory(&self) -> Result<(), FileManagerError> {
        match std::fs::remove_dir_all(&self.temporary_directory) {
            Ok(()) => Ok(()),
            Err(e) => Err(FileManagerError::UnaccessableDirectory {
                reason: e.to_string(),
            }),
        }
    }

    pub fn rename(tmp: &std::path::Path, old: &PathBuf) -> std::io::Result<()> {
        std::fs::rename(tmp, old)
    }
}

impl std::default::Default for FileManager {
    fn default() -> Self {
        let name = uuid::Uuid::new_v4().to_string();
        Self::new(std::path::PathBuf::from(name))
    }
}

impl Drop for FileManager {
    fn drop(&mut self) {
        self.remove_temporary_directory().ok();
    }
}
