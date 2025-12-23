use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("Kafka error: {0}")]
    Kafka(String),

    #[error("Database error: {0}")]
    Database(String),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Connection error: {0}")]
    Connection(String),

    #[error("Invalid input: {0}")]
    InvalidInput(String),

    #[error("Operation timeout")]
    Timeout,

    #[error("Not connected to Kafka cluster")]
    NotConnected,

    #[error("Terminal error: {0}")]
    Terminal(String),

    #[error("Validation error in {field}: {message}")]
    Validation { field: String, message: String },
}

impl From<rdkafka::error::KafkaError> for AppError {
    fn from(err: rdkafka::error::KafkaError) -> Self {
        AppError::Kafka(err.to_string())
    }
}

impl From<sqlx::Error> for AppError {
    fn from(err: sqlx::Error) -> Self {
        AppError::Database(err.to_string())
    }
}

impl From<toml::de::Error> for AppError {
    fn from(err: toml::de::Error) -> Self {
        AppError::Config(err.to_string())
    }
}

pub type AppResult<T> = Result<T, AppError>;
