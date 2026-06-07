use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
	#[error("Kafka error: {0}")]
	Kafka(#[from] rdkafka::error::KafkaError),

	#[error("Serialization error: {0}")]
	Serialization(#[from] serde_json::Error),

	#[error("Config error: {0}")]
	Config(#[from] config::ConfigError),
}


pub type Result<T> = std::result::Result<T, AppError>;