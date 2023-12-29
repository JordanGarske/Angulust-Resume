use rocket:: serde::json::Json;
use diesel::RunQueryDsl;
use crate::{Db, schema::client::{self}, models::user::User};
//deccription: creates a account for the user
//return: gives back a user Json back to the client
#[post("/sign-up",format = "json", data="<user>")]
pub async fn create_user(user:Json<User>, conn:Db) -> Json<User>{
    let user = user.into_inner();
    conn.run(move |c| {
        diesel::insert_into(client::table)
        .values(user).get_result::<User>(c)
    })
    .await
    .map(Json)
    .expect("the information was not valid")
    }


