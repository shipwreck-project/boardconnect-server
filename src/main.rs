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

    match Iron::new(chain).http("localhost:3000") {
        Ok(_o) => println!("Server is Starting in Port 3000"),
        Err(e) => println!("Server start failed due to {}", e),
    }
}
