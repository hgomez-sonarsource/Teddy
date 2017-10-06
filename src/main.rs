extern crate iron;
extern crate router;

use iron::prelude::*;

use router::Router;

mod handlers;
use handlers::{hello_handler, ping_handler};

mod auth;
use auth::AuthChecker;

mod responsetime;
use responsetime::ResponseTime;

extern crate time;
extern crate url;


fn main() {
    let ip = "0.0.0.0";
    let port = 3000;
    let addr = format!("{}:{}", ip, port);
    let auth = "teddy:rocks";

    let mut router = Router::new();           // Alternative syntax:
    router.get("/ping", ping_handler, "index");        // let router = router!(index: get "/" => handler,
    router.get("/hello", hello_handler, "query");  //                      query: get "/:query" => handler);


    let mut chain = Chain::new(router);
    chain.link_before(ResponseTime);
    chain.link_before(AuthChecker::new(auth));
    chain.link_after(ResponseTime);

    Iron::new(chain).http(&addr).unwrap();
}
