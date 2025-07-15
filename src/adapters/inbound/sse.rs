use crate::app::{domain::PushNotification, ports::NotificationPublisherPort};
use rocket::response::stream::{Event, EventStream};
// use rocket::serde::json::Json;
use rocket::{Shutdown, State, get, routes};
use std::sync::Mutex;
use tokio::sync::broadcast::Sender;

pub struct SsePublisher {
    tx: Sender<PushNotification>,
}

impl SsePublisher {
    pub fn new(tx: Sender<PushNotification>) -> Self {
        Self { tx }
    }
}

#[async_trait::async_trait]
impl NotificationPublisherPort for SsePublisher {
    async fn publish(&self, notification: PushNotification) {
        let _ = self.tx.send(notification);
    }
}

#[get("/events/sse")]
pub async fn sse_endpoint(
    queue: &State<Mutex<Sender<PushNotification>>>,
    mut end: Shutdown,
) -> EventStream![] {
    let mut rx = queue.lock().unwrap().subscribe();
    EventStream! {
        loop {
            tokio::select! {
                msg = rx.recv() => {
                    if let Ok(notification) = msg {
                        let json = serde_json::to_string(&notification).unwrap();
                        yield Event::data(json);
                    }
                }
                _ = &mut end => break,
            }
        }
    }
}

pub fn rocket_routes() -> Vec<rocket::Route> {
    routes![sse_endpoint]
}
