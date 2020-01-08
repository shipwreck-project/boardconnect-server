extern crate iron;
extern crate router;

mod index;

use router::Router;

pub fn new() -> Router {
  let mut routes = Router::new();
  index::routes(&mut routes);
  routes
}
