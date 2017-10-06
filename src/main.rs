extern crate iron;

use iron::prelude::*;
use iron::status;

fn main() {
    Iron::new(|_: &mut Request| {
        Ok(Response::with((status::Ok, "Teddy says: Hello World!")))
    }).http("localhost:3000").unwrap();
}
