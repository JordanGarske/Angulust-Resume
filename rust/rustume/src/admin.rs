//rocket crates
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
//diesel crates
use diesel::{QueryDsl,RunQueryDsl, result::Error};
//rustume crates
use crate::Db;
use crate::{schema::client::{self}, models::user::User};
use crate::authentication::cookie::cookie_thief;
pub(crate) mod get;
use crate::admin::get::get_users;
//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//routing for rocket

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users]
}
pub struct Admin<'r>(&'r str);

#[derive(Debug)]
pub enum ApiKeyError {
    Missing,
    Invalid,
}

#[rocket::async_trait]
impl<'r> FromRequest<'r> for Admin<'r> {
    type Error = ApiKeyError;

    async fn from_request(req: &'r Request<'_> ) -> Outcome<Self, Self::Error> {
        
        let db_outcome = req.guard::<Db>().await;
        let conn: Db = db_outcome.unwrap();
        let id = cookie_thief(req.cookies());
        let item:Result<User, Error>  = conn.run(move |conn| {
            //find user  information
            client::table.find(id).first::<User>(conn)
        })
        .await;

        match item {
            Ok(key) if key.admin_privilege == true => Outcome::Success(Admin("is a admin")),
            Err(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
            Ok(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
        }
    }

}
