use rdkafka::config::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use std::time::Duration;

use crate::error::Result;

pub struct KafkaProducer {
    producer: FutureProducer,
    topic: String,
}

impl KafkaProducer {
    pub fn new(brokers: &str, topic: &str) -> Result<Self> {
        let producer: FutureProducer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .create()?;

        Ok(Self {
            producer,
            topic: topic.to_string(),
        })
    }

    pub async fn send(&self, key: &str, payload: &str) -> Result<()> {
        let result = self
            .producer
            .send(
                FutureRecord::to(&self.topic).key(key).payload(payload),
                Duration::from_secs(5),
            )
            .await;

        match result {
            Ok(delivery) => {
                println!("Sent! partition={} offset={}", delivery.partition, delivery.offset);
                Ok(())
            }
            Err((err, _)) => Err(err.into()),
        }
    }
}
