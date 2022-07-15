pub struct ConnectionInformation {
    pub host: &'static str,
    pub port: &'static str
}

impl ConnectionInformation {
    pub fn to_address(&self) -> String {
        format!("{}:{}", self.host, self.port)
    }
}
