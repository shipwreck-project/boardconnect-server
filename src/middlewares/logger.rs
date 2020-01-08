extern crate iron;

use crate::utils::time;
use iron::prelude::*;
use iron::{AroundMiddleware, Handler};

pub struct Logger;

struct LoggerHandler<H: Handler> {
  logger: Logger,
  handler: H,
}

impl Logger {
  pub fn new() -> Logger {
    Logger {}
  }

  pub fn log(&self, req: &Request, res: Result<&Response, &IronError>, time: i128) {
    println!("Req: {:?}\nRes: {:?}\nTook: {}", req, res, time);
  }
}

impl<H: Handler> Handler for LoggerHandler<H> {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let entry = time::get_current_time_in_ns();
    let res = self.handler.handle(req);
    self
      .logger
      .log(req, res.as_ref(), time::get_current_time_in_ns() - entry);
    res
  }
}

impl AroundMiddleware for Logger {
  fn around(self, handler: Box<dyn Handler>) -> Box<dyn Handler> {
    Box::new(LoggerHandler {
      logger: self,
      handler,
    }) as Box<dyn Handler>
  }
}
