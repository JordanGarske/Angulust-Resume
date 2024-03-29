#[macro_use] extern crate rocket;
use rocket::{fs::FileServer, fs::relative};
use rocket_sync_db_pools::database;
//the raw struct that are produce by diesel
mod schema;
//models for all the table for the database
mod models;
// for all the autehntication like loging in,cookies, and sign up
mod authentications;
use authentications::authentication;
//routes for special admin privillibes
mod admin_access;
//client routes for user only privilliges
mod client_access;
#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    rocket::build()
    .attach(Db::fairing())
    .mount("/admin", admin_access::admin::routes())
    .mount("/client_access", client_access::routes())
    .mount("/", authentication::routes())
    .mount("/", FileServer::from(relative!("static/resup")))
}

