use crate::schema::users;

#[derive(Queryable)]
pub struct Users {
  pub user_key: u64,
  pub id: String,
  pub pw: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct NewUser<'a> {
  pub id: &'a str,
  pub pw: &'a str,
}
