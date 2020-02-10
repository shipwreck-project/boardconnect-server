use crate::models::*;

use crate::models;
use crate::services::Pool;
use diesel::prelude::*;
use diesel::MysqlConnection;

pub struct Service {
  pool: Pool,
}

impl Service {
  pub fn new(self, pool: Pool) -> Service {
    Service { pool: pool }
  }

  pub fn connection_test_insert(self) -> super::DieselResult<usize> {
    use crate::schema::users;

    let new_user = NewUser {
      id: "asdfasdf",
      pw: "asdfasdf",
    };

    let conn: &MysqlConnection = &self.pool.get().unwrap();

    let res = diesel::insert_into(users::table)
      .values(&new_user)
      .execute(conn)
      .expect("Error saving new User");

    Ok(res)
  }

  pub fn connection_test_select(self) -> super::DieselResult<Vec<models::Users>> {
    use crate::schema::users::dsl::*;

    let conn: &MysqlConnection = &self.pool.get().unwrap();

    let results = users
      .limit(5)
      .load::<Users>(conn)
      .expect("Error loading Users");

    Ok(results)
  }
}
