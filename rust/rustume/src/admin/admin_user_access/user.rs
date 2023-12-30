use rocket::serde::json::{Json, self};
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods };
use crate::{Db, schema::{personal_reference, client}, models::{user::{User, self}, personal_reference::{Insert__Personal_Reference, Personal_Reference}}};
use crate::admin::Admin;


#[get("/get_users")]
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


#[post("/reference_permission_access",format = "json", data="<id>")]
pub async fn give_reference_access(id:Json<i32>,_admin: Admin<'_>, conn:Db) -> Json<bool>{
        let record = Insert__Personal_Reference{client_id:  id.into_inner(), elucidation:"".to_string()};
        let result = conn.run(move |c| {
            diesel::insert_into(personal_reference::table)
            .values(record).get_result::<Personal_Reference>(c)
        })
        .await;
    match result {
        Ok(value) =>  {
            let result = conn.run(move |c|
                {
                diesel::update(client::table).filter(client::client_id.eq(value.client_id)).set(client::personal_review_id.eq(value.personal_review_id)).execute(c)
                }
                ).await;
            return Json(false) 
        },
        Err(_) => return Json(false)               
    }
 }
    
    

