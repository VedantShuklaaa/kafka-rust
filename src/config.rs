use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
pub struct Settings {
    pub kafka: KafkaSettings,
}

#[derive(Deserialize, Debug, Clone)]
pub struct KafkaSettings {
    pub brokers: String,
    pub topic: String,
    pub group_id: String,
}

impl Settings {
    pub fn load() -> Result<Self, config::ConfigError> {
        config::Config::builder()
            .add_source(config::File::with_name("config/default"))
            .add_source(config::Environment::with_prefix("APP").separator("__"))
			.build()?
			.try_deserialize()
    }
}
