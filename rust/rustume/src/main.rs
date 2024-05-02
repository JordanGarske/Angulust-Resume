#[macro_use] extern crate rocket;
use rocket::{fs::FileServer, fs::relative};
use rocket_sync_db_pools::database;
use rocket::tokio::sync::broadcast::channel;
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
use crate::client_access::personal_info;
mod chat;
#[database("my_db")]
pub struct Db(diesel::PgConnection);

#[launch]
fn rocket() -> _ {
    //active chat rooms
    rocket::build()
    .attach(Db::fairing())
    .manage(channel::<chat::room::Message>(1024).0)
    .mount("/admin", admin_access::admin::routes())
    .mount("/client_access", personal_info::routes())
    .mount("/", authentication::routes())
    .mount("/", chat::room::routes())
    .mount("/", FileServer::from(relative!("static/resup")))
}

#[cfg(test)]
    mod test {
        use super::rocket;
        use rocket::local::blocking::Client;
        use rocket::http::Status;
    
        #[test]
        fn hello_world() {
            let client = Client::tracked(rocket()).expect("valid rocket instance");
            let response = client.get("/").dispatch();
            assert_eq!(response.status(), Status::Ok);
        }
    }