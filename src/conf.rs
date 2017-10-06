extern crate rustc_serialize;

use self::rustc_serialize::json::Json;
use std::fs::File;
use std::io::Read;
use std::path::Path;

pub struct Configuration{
    pub host: String,
    pub port: u16,
    pub authentication:String,
}

pub fn get_config() -> Configuration {
    if Path::new("config.json").exists() {
        get_config_from_file()
    } else {
        get_default_config()
    }
}

fn get_config_from_file()  -> Configuration {
    let mut file = File::open("config.json").unwrap();
    let mut data = String::new();
    file.read_to_string(&mut data).unwrap();

    let json = Json::from_str(&data).unwrap();
    let user = json.find_path(&["Authentication", "User"]).unwrap().as_string().unwrap();
    let password = json.find_path(&["Authentication", "Password"]).unwrap().as_string().unwrap();
    let port = json.find("Port").unwrap().as_string().unwrap();
    let port_u16: u16 = port.parse().unwrap();
    Configuration {
        host: String::from("0.0.0.0"),
        port: port_u16,
        authentication: format!("{}:{}", user, password)
    }
}

fn get_default_config() -> Configuration {
    Configuration {
        host: String::from("0.0.0.0"),
        port: 3000,
        authentication: String::from("teddy:rocks"),
    }
}

pub fn get_address(config: Configuration) -> String{
    format!("{}:{}", config.host, config.port)
}