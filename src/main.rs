extern crate iron;
extern crate router;

use iron::prelude::*;

use router::Router;

mod handlers;
use handlers::{hello_handler, ping_handler, download_handler};

mod auth;
use auth::AuthChecker;

mod responsetime;
use responsetime::ResponseTime;

mod conf;
use conf::get_config;
use conf::get_address;

extern crate time;
extern crate url;


fn main() {
    let conf = get_config();

    let mut router = Router::new();
    router.get("/ping", ping_handler, "ping");
    router.get("/hello", hello_handler, "hello");
    router.get("/download", download_handler, "download");

    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_before(AuthChecker::new(&conf.authentication));
    chain.link_after(ResponseTime);

    Iron::new(chain).http(&get_address(conf)).unwrap();
}
