extern crate iron;
extern crate params;


use iron::prelude::*;
use std::path::Path;
use std::fs::File;

use hyper::header::{ContentDisposition, DispositionType, DispositionParam, Charset};

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
            let file_name = Path::new(path).file_name();
            let mut response = Response::with((iron::status::Ok, file));
            response.headers.set(ContentDisposition {
                disposition: DispositionType::Attachment,
                parameters: vec![DispositionParam::Filename(
                    Charset::Iso_8859_1, // The character set for the bytes of the filename
                    None, // The optional language tag (see `language-tag` crate)
                    Vec::from(format!("{:?}", file_name.unwrap()).replace("\"", "")) // the actual bytes of the filename
                )]
            });;
            Ok(response)
        },
        _ => Ok(Response::with(iron::status::NotFound)),
    }
}
