use futures::StreamExt;
use rdkafka::Message;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};

use crate::error::Result;

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(brokers: &str, group_id: &str, topics: &[&str]) -> Result<Self> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", brokers)
            .set("group.id", group_id)
            .set("auto.offset.reset", "earliest")
            .set("enable.auto.commit", "false")
            .create()?;

        consumer.subscribe(topics)?;

        Ok(Self { consumer })
    }

    pub async fn run<F, Fut>(&self, handler: F) -> Result<()>
    where
        F: Fn(String, String) -> Fut,
        Fut: std::future::Future<Output = Result<()>>,
    {
        println!("Consumer started, waiting for messages...");
        let mut stream = self.consumer.stream();

        while let Some(message) = stream.next().await {
            match message {
                Ok(msg) => {
                    let key = msg
                        .key_view::<str>()
                        .unwrap_or(Ok(""))
                        .unwrap_or("")
                        .to_string();

                    let payload = msg
                        .payload_view::<str>()
                        .unwrap_or(Ok(""))
                        .unwrap_or("")
                        .to_string();

                    println!(
                        "Received | partition={} offset={} key={} payload={}",
                        msg.partition(),
                        msg.offset(),
                        key,
                        payload
                    );

                    if let Err(e) = handler(key, payload).await {
                        eprintln!("Handler error: {}", e);
                    } else {
                        let _ = self
                            .consumer
                            .commit_message(&msg, rdkafka::consumer::CommitMode::Async);
                    }
                }
                Err(e) => eprintln!("Consumer error: {}", e),
            }
        }

        Ok(())
    }
}

