use std::{fs, io, path};

#[derive(serde::Deserialize, Debug)]
pub struct Config {
    pub tasks: Vec<Task>,
}

#[derive(serde::Deserialize, Debug)]
pub struct Task {
    pub name: String,
    pub command: String,
    pub output: Option<bool>,
}

#[derive(Debug)]
pub enum ConfigError {
    FilesystemError(io::Error),
    Invalid(serde_yml::Error),
}

impl Config {
    pub fn read(path: &path::Path) -> Result<Box<Config>, ConfigError> {
        let file_contents = fs::read_to_string(path)?;
        let config: Config = serde_yml::from_str(&file_contents)?;

        Ok(Box::new(config))
    }
}

impl From<io::Error> for ConfigError {
    fn from(value: io::Error) -> Self {
        ConfigError::FilesystemError(value)
    }
}

impl From<serde_yml::Error> for ConfigError {
    fn from(value: serde_yml::Error) -> Self {
        ConfigError::Invalid(value)
    }
}
