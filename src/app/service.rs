use super::{
    domain::PushNotification,
    ports::{NotificationPublisherPort, NotificationSubscriberPort},
};
use std::sync::Arc;

pub struct NotificationService {
    publisher: Arc<dyn NotificationPublisherPort>,
}

impl NotificationService {
    pub fn new(publisher: Arc<dyn NotificationPublisherPort>) -> Self {
        Self { publisher }
    }

    pub async fn process_incoming(&self, notification: PushNotification) {
        println!("📢 Broadcasting notification: {:?}", notification);
        self.publisher.publish(notification).await;
    }

    pub async fn run_subscriber<S>(self: Arc<Self>, subscriber: Arc<S>)
    where
        S: NotificationSubscriberPort + 'static,
    {
        subscriber
            .subscribe(move |notification| {
                let svc = self.clone();
                tokio::spawn(async move {
                    svc.process_incoming(notification).await;
                });
            })
            .await;
    }
}
