pub mod test;

use diesel::r2d2::ConnectionManager;
use diesel::MysqlConnection;
use std::env;

type DieselResult<T> = Result<T, diesel::result::Error>;

pub type Pool = r2d2::Pool<ConnectionManager<MysqlConnection>>;

pub fn init_pool() -> Pool {
  let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
  let manager = ConnectionManager::<MysqlConnection>::new(database_url);
  let pool = r2d2::Pool::builder()
    .build(manager)
    .expect("Failed to create pool.");
  pool
}
