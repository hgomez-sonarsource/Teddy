extern crate iron;

use iron::prelude::*;
use iron::status;
use iron::{BeforeMiddleware, AfterMiddleware, typemap};
use time::precise_time_ns;
use std::fmt;
use std::error::Error;

extern crate time;
extern crate url;

struct ResponseTime;

impl typemap::Key for ResponseTime { type Value = u64; }


/*** StringError ***/
#[derive(Debug)]
pub struct StringError(pub String);

impl fmt::Display for StringError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        fmt::Display::fmt(&self.0, f)
    }
}

impl Error for StringError {
    fn description(&self) -> &str {
        &self.0
    }
}

/*** Auth&Co ***/
pub struct AuthChecker {
    username: String,
    password: String
}

impl AuthChecker {
    pub fn new(s: &str) -> AuthChecker {
        let parts = s.splitn(2, ':').collect::<Vec<&str>>();
        AuthChecker {
            username: parts[0].to_owned(),
            password: parts[1].to_owned()
        }
    }
}

impl BeforeMiddleware for AuthChecker {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        use iron::headers::{Authorization, Basic};

        match req.headers.get::<Authorization<Basic>>() {
            Some(&Authorization(Basic{ ref username, ref password })) => {
                if username == self.username.as_str() && password == &Some(self.password.clone()) {
                    Ok(())
                } else {
                    Err(IronError {
                        error: Box::new(StringError("authorization error".to_owned())),
                        response: Response::with((status::Unauthorized, "Wrong username or password."))
                    })
                }
            }
            None => {
                let mut resp = Response::with(status::Unauthorized);
                resp.headers.set_raw("WWW-Authenticate", vec![b"Basic realm=\"main\"".to_vec()]);
                Err(IronError {
                    error: Box::new(StringError("authorization error".to_owned())),
                    response: resp
                })
            }
        }
    }
}


impl BeforeMiddleware for ResponseTime {
    fn before(&self, req: &mut Request) -> IronResult<()> {
        println!("headers are {}", req.headers);
        req.extensions.insert::<ResponseTime>(precise_time_ns());
        Ok(())
    }
}

impl AfterMiddleware for ResponseTime {
    fn after(&self, req: &mut Request, res: Response) -> IronResult<Response> {
        let delta = precise_time_ns() - *req.extensions.get::<ResponseTime>().unwrap();
        println!("Request took: {} ms", (delta as f64) / 1000000.0);
        Ok(res)
    }
}

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn main() {
    let ip = "0.0.0.0";
    let port = 3000;
    let addr = format!("{}:{}", ip, port);
    let auth = "teddy:rocks";

    let mut chain = Chain::new(hello_world);
    chain.link_before(ResponseTime);
    chain.link_before(AuthChecker::new(auth));
    chain.link_after(ResponseTime);
    Iron::new(chain).http(&addr).unwrap();
}
