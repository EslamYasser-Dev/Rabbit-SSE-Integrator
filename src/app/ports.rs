use super::domain::PushNotification;

#[async_trait::async_trait]
pub trait NotificationPublisherPort: Send + Sync {
    async fn publish(&self, notification: PushNotification);
}

#[async_trait::async_trait]
pub trait NotificationSubscriberPort: Send + Sync {
    async fn subscribe<F>(&self, handler: F)
    where
        F: Fn(PushNotification) + Send + Sync + 'static;
}
