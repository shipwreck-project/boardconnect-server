extern crate iron;
extern crate time;

use iron::prelude::*;
mod middlewares;
use middlewares::logger::{Logger, LoggerMode};

fn hello_world(_: &mut Request) -> IronResult<Response> {
    Ok(Response::with((iron::status::Ok, "Hello World")))
}

fn main() {
    let mut chain = Chain::new(hello_world);
    chain.link_around(Logger::new(LoggerMode::Tiny));
    Iron::new(chain).http("localhost:3000").unwrap();
}
