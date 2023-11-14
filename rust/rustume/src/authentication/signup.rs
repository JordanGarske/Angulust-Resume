use rocket:: serde::json::Json;
use diesel::RunQueryDsl;
use crate::{Db, schema::client::{self}, models::user::User};
//take in infromation about about a user signing up
//return 
#[post("/sign-up",format = "json", data="<user>")]
pub async fn create_user(user:Json<User>, conn:Db) -> Json<User>{
    let user = user.into_inner();
    let  new_user = User{
        client_id: user.client_id,
        first_name: user.first_name.to_string() ,
        last_name: user.last_name.to_string(),
        client_password:user.client_password.to_string(),
        email: user.email.to_string(),
        admin_privilege: false,
    };
    conn.run(move |c| {
        diesel::insert_into(client::table)
        .values(new_user).get_result::<User>(c)
    })
    .await
    .map(Json)
    .expect("the information was not valid")
    }


