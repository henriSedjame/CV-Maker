use thiserror::Error;

#[derive(Error, Debug)]
pub enum CVErrors {
    #[error("File not found")]
    FileNotFound(#[from] Box<dyn std::error::Error>),

    #[error("Error occurred deserializing yaml file: {0}")]
    YamlDeserializationError(#[from] serde_yaml::Error),
}