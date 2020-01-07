extern crate iron;
extern crate time;

use iron::prelude::*;
use iron::{AroundMiddleware, Handler};
use time::PrimitiveDateTime;

pub enum LoggerMode {
  Silent,
  Tiny,
  Large,
}

pub struct Logger {
  mode: LoggerMode,
}

struct LoggerHandler<H: Handler> {
  logger: Logger,
  handler: H,
}

impl Logger {
  pub fn new(mode: LoggerMode) -> Logger {
    Logger { mode }
  }

  pub fn log(&self, req: &Request, res: Result<&Response, &IronError>, time: i128) {
    match self.mode {
      LoggerMode::Silent => {}
      LoggerMode::Tiny => println!("Req: {:?}\nRes: {:?}\nTook: {}", req, res, time),
      LoggerMode::Large => println!(
        "Request: {:?}\nResponse: {:?}\nResponse-Time: {}",
        req, res, time
      ),
    }
  }
}

impl<H: Handler> Handler for LoggerHandler<H> {
  fn handle(&self, req: &mut Request) -> IronResult<Response> {
    let entry = get_currnet_time_in_ns();
    let res = self.handler.handle(req);
    self
      .logger
      .log(req, res.as_ref(), get_currnet_time_in_ns() - entry);
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

fn get_currnet_time_in_ns() -> i128 {
  let duration = PrimitiveDateTime::now() - PrimitiveDateTime::unix_epoch();
  duration.whole_nanoseconds()
}
