extern crate actix_web;
#[macro_use]
extern crate diesel;
extern crate dotenv;

mod models;
mod schema;

use actix_web::{get, middleware, App, HttpServer, Responder};
use diesel::connection::Connection;
use diesel::mysql::MysqlConnection;
use diesel::prelude::*;
use dotenv::dotenv;
use env_logger;
use models::*;
use std::env;

fn establish_connection() -> MysqlConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    MysqlConnection::establish(&database_url)
        .expect(&format!("Error connecting to {}", database_url))
}

fn connection_test_insert(conn: &MysqlConnection) {
    use crate::schema::users;

    let new_user = NewUser {
        id: "asdfasdf",
        pw: "asdfasdf",
    };

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(conn)
        .expect("Error saving new User");
}

fn connection_test_select(conn: &MysqlConnection) {
    use crate::schema::users::dsl::*;
    let results = users
        .limit(5)
        .load::<Users>(conn)
        .expect("Error loading Users");

    println!("Displaying {} Users", results.len());
    for user in results {
        println!("{}", user.user_key);
        println!("{}", user.id);
        println!("{}", user.pw);
    }
}

#[get("/")]
async fn index() -> impl Responder {
    format!("Hello world!")
}

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "actix_web=info");
    env_logger::init();

    let conn = establish_connection();

    connection_test_insert(&conn);
    connection_test_select(&conn);

    HttpServer::new(|| {
        App::new()
            .service(index)
            .wrap(middleware::Logger::default())
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
