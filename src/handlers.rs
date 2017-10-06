extern crate iron;

use iron::Response;
use iron::Request;
use iron::IronResult;

pub fn hello_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

pub fn ping_handler(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "pong")))
}
