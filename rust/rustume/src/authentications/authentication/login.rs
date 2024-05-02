use rocket::{http::CookieJar, serde::json::Json};
use crate::{Db, models::{client::UserLoginInfo, credential::CredentialApproval, reviews::InsertReview}};
use crate::authentications::authentication::cookie::bake_cookie;
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
    match result 
    {
        Ok(value) => 
        {
           let id = value.id;
            let admin = value.admin_privilege;
            bake_cookie(jar, "user_id".to_string(), id.to_string());
            bake_cookie(jar, "user_name".to_string(), value.first_name.clone()+" " +&value.last_name);
            let review_result = conn.run(move |temp_conn| {
                InsertReview::get_client_review(value,temp_conn)
            }).await;

            match review_result{
                Ok(review) => return Json(CredentialApproval::new(true, admin,Some(review.id) ))  ,
                Err(_) => return Json(CredentialApproval::new(true, admin,None ))   
            }
            
            
        },
        Err(_) => return Json(CredentialApproval::new(false, false,None ))               
    }
}