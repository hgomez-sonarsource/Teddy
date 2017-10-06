extern crate iron;
extern crate params;

use iron::prelude::*;
use std::path::Path;
use std::fs::File;

pub fn hello_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

pub fn ping_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "pong")))
}

pub fn download_handler(req: &mut Request) -> IronResult<Response> {
    use self::params::{Params, Value};

    let map = req.get_ref::<Params>().unwrap();

    match map.find(&["path"]) {
        Some(&Value::String(ref path)) if Path::new(path).exists() && Path::new(path).is_file() => {
            let file = File::open(path).unwrap();
            Ok(Response::with((iron::status::Ok, file)))
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}
