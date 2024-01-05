use diesel::SelectableHelper;
//rocket crates
use rocket::http::Status;
use rocket::request::{Outcome, Request, FromRequest};
//diesel crates
use diesel::{QueryDsl,RunQueryDsl, result::Error};
//rustume crates
use crate::Db;
use crate::models::client::AdminUser;
use crate::schema::clients::{self};
use crate::authentication::cookie::cookie_thief;
pub(crate) mod user_routes;
use crate::admin::user_routes::user::{give_reference_access, get_clients};
pub(crate) mod review_routes;
use crate::admin::review_routes::review::{delete_review,get_user_reviews};
//-----------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------
//routing for rocket

pub fn routes() -> Vec<rocket::Route> {
    routes![get_user_reviews,get_clients,give_reference_access,delete_review]
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
            clients::table.find(id).select(AdminUser::as_select()).first(temp_conn)
        })
        .await;
        print!("{}",id);
        match item {
            Ok(key) if key.admin_privilege == true => Outcome::Success(Admin("is a admin")),
            Err(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
            Ok(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
        }
    }

}
