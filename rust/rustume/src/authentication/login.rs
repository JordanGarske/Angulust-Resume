use rocket::{http::CookieJar, serde::json::Json};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods};
use crate::{Db, schema::client::{self, client_password, email}, models::user::{User,UserClientInfo}};
use crate::authentication::cookie::bake_cookie;
//deccription: logs user into the website
//return: gives back a cookie and user Json back to the client
#[post("/login",format = "json", data="<user>")]
pub async fn login_user(user:Json<User>, conn:Db,  jar: &CookieJar<'_>) -> Json<UserClientInfo>{
    let user = user.into_inner();
    conn.run(move |conn| {
        //find user  information
        let client_indfication = client_password.eq(user.client_password)
                                .and(email.eq(user.email));
        client::table::filter(client::table, client_indfication)
        .first::<User>(conn)
    })
    .await
    .map(|value:User|{
            //if info on user is found than send a cookie and there info back to them  
            bake_cookie(jar, "user_id".to_string(), value.client_id.to_string());
            Json(
                UserClientInfo
                {
                    first_name: value.client_password,
                    last_name: value.last_name,     
                    email: value.email
                }
            )
    })
    .unwrap_or(   
            Json(
                UserClientInfo{
                    first_name: "fail".to_string(), 
                    last_name: "fail".to_string(),
                    email: "f".to_string()
                })
                )
}