// use rocket:: serde::json::Json;
// use diesel::RunQueryDsl;
// use crate::{Db, schema::client::self, models::user::User};
pub(crate) mod login;
pub(crate) mod cookie;
pub(crate) mod signup;
use crate::authentications::authentication::signup::create_user;
use crate::authentications::authentication::login::login_user;
//
pub fn routes() -> Vec<rocket::Route> {
    routes![create_user, login_user]
}
