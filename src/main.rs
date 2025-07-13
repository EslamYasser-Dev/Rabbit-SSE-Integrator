extern crate rocket;
mod adapters;
mod app;
use adapters::inbound::sse::{SsePublisher, rocket_routes};
use adapters::outbound::rabbitmq::RabbitMqSubscriber;
use app::service::NotificationService;
use std::sync::{Arc, Mutex};
use tokio::sync::broadcast;

#[rocket::main]
async fn main() {
    let rabbit_url = String::from("amqp://admin:admin@mas10:5672/%2f");
    let source_queue = String::from("push_queue");
    let (tx, _rx) = broadcast::channel(100);

    // el publisher da yast5dm HTTP SSE
    let publisher = Arc::new(SsePublisher::new(tx.clone()));

    // el subscriber da yast5dm RabbitMQ
    let subscriber = Arc::new(RabbitMqSubscriber::new(&rabbit_url, &source_queue));

    let service = Arc::new(NotificationService::new(publisher.clone()));

    // Start RabbitMQ consumer in another thread
    let svc_clone = service.clone();
    tokio::spawn(async move {
        svc_clone.run_subscriber(subscriber).await;
    });

    // Start Rocket server
    println!("🚀 Rocket SSE server is running on http://127.0.0.1:8761");
    rocket::build()
        .manage(Mutex::new(tx))
        .mount("/", rocket_routes())
        .launch()
        .await
        .expect("runtime error");
}
