use kafka_demo::kafka::KafkaConsumer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let consumer = KafkaConsumer::new("localhost:9092", "order-processor", &["trades"])?;

    consumer
        .run(|key, payload| async move {
            println!("Processing → key={} payload={}", key, payload);
            Ok::<(), kafka_demo::error::AppError>(())
        })
        .await?;

    Ok(())
}
