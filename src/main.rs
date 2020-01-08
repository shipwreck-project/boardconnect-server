extern crate iron;
extern crate time;

mod middlewares;
mod routes;
mod utils;

use iron::prelude::*;
use middlewares::logger::Logger;

fn main() {
    let mut chain = Chain::new(routes::new());
    chain.link_around(Logger::new());

    match Iron::new(chain).http("localhost:3000") {
        Ok(_o) => println!("Server is Starting in Port 3000"),
        Err(e) => println!("Server is failed due to {}", e),
    }
}
