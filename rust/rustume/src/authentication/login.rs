use rocket::{http::CookieJar, serde::json::Json, figment::value};
use crate::{Db, models::{user::{User,UserLoginInfo}, credential::CredentialApproval}};
use crate::authentication::cookie::bake_cookie;
//deccription: logs user into the website
//return: gives back a cookie and user Json back to the client
#[post("/login",format = "json", data="<user>")]
pub async fn login_user(user:Json<UserLoginInfo>, conn:Db,  jar: &CookieJar<'_>) -> Json<CredentialApproval>{
    let user = user.into_inner();
    let result = conn.run(move |temp_conn| {
        //find user  information
        user.login(temp_conn)
    })
    .await;
    match result {
        Ok(value) => {
            bake_cookie(jar, "user_id".to_string(), value.client_id.to_string());
            Json(CredentialApproval::new(true,value.admin_privilege))
        },
        Err(_) => return Json(CredentialApproval::new(false, false))               
    }
}