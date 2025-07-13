use lapin::{
    options::{BasicAckOptions, BasicNackOptions, QueueDeclareOptions, BasicConsumeOptions},
    types::FieldTable,
    Connection, ConnectionProperties, Consumer,
};
use futures_util::StreamExt;
use crate::app::{domain::PushNotification, ports::NotificationSubscriberPort};
use serde_json;

pub struct RabbitMqSubscriber {
    uri: String,
    queue: String,
}

impl RabbitMqSubscriber {
    pub fn new(uri: &str, queue: &str) -> Self {
        Self {
            uri: uri.to_string(),
            queue: queue.to_string(),
        }
    }
}

#[async_trait::async_trait]
impl NotificationSubscriberPort for RabbitMqSubscriber {
    async fn subscribe<F>(&self, handler: F)
    where
        F: Fn(PushNotification) + Send + Sync + 'static,
    {
        let conn = Connection::connect(&self.uri, ConnectionProperties::default())
            .await
            .expect("❌ RabbitMQ connection failed");
        let channel = conn.create_channel().await.expect("❌ Failed to create channel");

        channel
            .queue_declare(
                &self.queue,
                QueueDeclareOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("❌ Queue declare failed");

        let mut consumer: Consumer = channel
            .basic_consume(
                &self.queue,
                "rocket_consumer",
                BasicConsumeOptions::default(),
                FieldTable::default(),
            )
            .await
            .expect("❌ Failed to start consumer");

        println!("📥 RabbitMQ subscriber listening on '{}'", &self.queue);

        while let Some(result) = consumer.next().await {
            match result {
                Ok(delivery) => {
                    match serde_json::from_slice::<PushNotification>(&delivery.data) {
                        Ok(notification) => {
                            println!("✅ Received: {:?}", notification);
                            // ✅ Call handler directly
                            (handler)(notification);

                            if let Err(e) = delivery.ack(BasicAckOptions::default()).await {
                                eprintln!("❌ Failed to ACK message: {:?}", e);
                            }
                        }
                        Err(e) => {
                            eprintln!("⚠️ Failed to deserialize message: {:?}", e);
                            eprintln!("📦 Raw: {:?}", String::from_utf8_lossy(&delivery.data));
                            if let Err(e) = delivery.nack(BasicNackOptions::default()).await {
                                eprintln!("❌ Failed to NACK bad message: {:?}", e);
                            }
                        }
                    }
                }
                Err(e) => {
                    eprintln!("❌ Error while consuming message: {:?}", e);
                }
            }
        }
    }
}
