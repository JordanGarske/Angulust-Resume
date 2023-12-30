use rocket::{http::CookieJar, serde::json::Json};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods, BoolExpressionMethods };
use crate::{Db, schema::client::{self}, models::user::{User, self}};
use crate::authentication::cookie::bake_cookie;
use crate::admin::Admin;


#[get("/get_user")]
pub async fn get_users(_admin: Admin<'_>, conn:Db) -> Json<Vec<User>>{
    let result = conn.run(move |c| {
        let users= client::table.select(client::all_columns).load::<User>(c);
        users.ok()
    })
    .await
    .map(Json)
    .expect("the information was not valid");
return  result;
    }


