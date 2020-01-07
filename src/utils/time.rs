extern crate time;

use time::PrimitiveDateTime;

pub fn get_current_time_in_ns() -> i128 {
  let duration = PrimitiveDateTime::now() - PrimitiveDateTime::unix_epoch();
  duration.whole_nanoseconds()
}
