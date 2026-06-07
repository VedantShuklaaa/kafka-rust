use kafka_demo::kafka::KafkaProducer;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let producer = KafkaProducer::new("localhost:9092", "trades")?;

    producer.send("BTCUSDT", "price=65000").await?;
    producer.send("ETHUSDT", "price=3400").await?;
    producer.send("SOLUSDT", "price=145").await?;

    Ok(())
}
