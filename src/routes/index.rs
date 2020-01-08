extern crate iron;
extern crate router;

use iron::prelude::*;
use router::Router;

pub fn routes(rtr: &mut Router) {
  rtr.get(
    "/",
    |_: &mut Request| Ok(Response::with((iron::status::Ok, "Hello World"))),
    "index",
  );
}
