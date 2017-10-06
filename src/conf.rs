pub struct Configuration{
    pub host: String,
    pub port: u16,
    pub authentication:String,
}

pub fn get_default_config() -> Configuration {
    Configuration {
        host: String::from("0.0.0.0"),
        port: 3000,
        authentication: String::from("teddy:rocks"),
    }
}

pub fn get_address(config: Configuration) -> String{
    format!("{}:{}", config.host, config.port)
}