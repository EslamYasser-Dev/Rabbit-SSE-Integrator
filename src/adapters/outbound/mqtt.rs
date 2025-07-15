use rumqttc::{Client, MqttOptions, QoS};
use std::time::Duration;

pub struct MqttSubscriber {
    host: String,
    client: Client,
    topic: String,
    client_id: String,
    username: String,
    password: String,
    keep_alive: u16,
    qos: u8,
    _reserved: (),
}

impl MqttSubscriber {
    pub fn new(
        host: String,
        topic: String,
        client_id: String,
        username: String,
        password: String,
        keep_alive: u16,
        qos: u8,
        _reserved: (),
    ) -> Self {
        let mut mqttoptions = MqttOptions::new(&client_id, &host, 1883);
        mqttoptions
            .set_credentials(username.clone(), password.clone())
            .set_keep_alive(Duration::from_secs(keep_alive as u64));

        let (client, _) = Client::new(mqttoptions, 10);

        Self {
            host,
            client,
            topic,
            client_id,
            username,
            password,
            keep_alive,
            qos,
            _reserved,
        }
    }

    pub fn subscribe(&mut self) {
        self.client.subscribe(
            &self.topic,
            QoS::try_from(self.qos)
                .unwrap_or(QoS::AtMostOnce)
                .expect("Invalid QoS"),
        )
    }

    pub fn unsubscribe(&mut self) {
        self.client.unsubscribe(&self.topic).unwrap();
    }

    pub fn disconnect(self) {
        drop(self.client);
    }
}
