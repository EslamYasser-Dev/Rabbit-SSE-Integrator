struct Config {
    address: String,
    port: u16,
    secret_key: String,
    service_name: String,
    publisher_url: String,
    subscriber_url: String,
}

struct ConfigBuilder {
    address: String,
    port: u16,
    secret_key: String,
    service_name: String,
}

impl ConfigBuilder {
    pub fn new(address: String, port: u16, secret_key: String, service_name: String) -> Self {
        ConfigBuilder {
            address,
            port,
            secret_key,
            service_name,
        }
    }

    pub fn build(self) -> Config {
        Config {
            address: self.address,
            port: self.port,
            secret_key: self.secret_key,
            service_name: self.service_name,
        }
    }
}

impl Config {
    pub fn new(address: String, port: u16, secret_key: String, service_name: String) -> Self {
        Config {
            address,
            port,
            secret_key,
            service_name,
        }
    }
}
