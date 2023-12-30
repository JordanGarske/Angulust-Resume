use diesel::SelectableHelper;
//rocket crates
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
//diesel crates
use diesel::{QueryDsl,RunQueryDsl, result::Error};
//rustume crates
use crate::Db;
use crate::models::user::AdminUser;
use crate::schema::client::{self};
use crate::authentication::cookie::cookie_thief;
pub(crate) mod admin_user_access;
use crate::admin::admin_user_access::user::{get_users,give_reference_access};
//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//routing for rocket

pub fn routes() -> Vec<rocket::Route> {
    routes![get_users,give_reference_access]
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
        let item:Result<AdminUser, Error>  = conn.run(move |temp_conn| {
            //find user  information
            client::table.find(id).select(AdminUser::as_select()).first(temp_conn)
        })
        .await;

        match item {
            Ok(key) if key.admin_privilege == true => Outcome::Success(Admin("is a admin")),
            Err(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
            Ok(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
        }
    }

}
