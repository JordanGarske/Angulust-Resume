use rocket:: serde::json::Json;
use diesel::RunQueryDsl;
use crate::{Db, schema::clients::{self}, models::client::{Client, UserSignUp}};
//deccription: creates a account for the user
//return: gives back a user Json back to the client
#[post("/sign-up",format = "json", data="<user>")]
pub async fn create_user(user:Json<UserSignUp>, conn:Db) -> Json<bool>{
    let user = user.into_inner();
    let result =conn.run(move |c| {
        diesel::insert_into(clients::table)
        .values(user).get_result::<Client>(c)
    })
    .await;
    match result {
        Ok(_) => return  Json(true),
        Err(_) => return Json(true)
        
    }
    
    }


