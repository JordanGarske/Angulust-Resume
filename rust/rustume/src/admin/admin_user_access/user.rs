use rocket::serde::json::Json;
use diesel::{RunQueryDsl, QueryDsl, ExpressionMethods };
use serde::Deserialize;
use crate::{Db, schema::{client,resume_reference}, models::{client::Client, personal_reference::{InsertPersonalReference,ResumeReference}}};
use crate::admin::Admin;

//deccription: give permission and make a recode to write a review for a user
//return: gives back confirmation that review was created
#[get("/get_users")]
pub async fn get_users(_admin: Admin<'_>, conn:Db) -> Json<Vec<Client>>{
    let result = conn.run(move |c| {
        let users= client::table.select(client::all_columns).load::<Client>(c);
        users.ok()
    })
    .await
    .map(Json)
    .expect("the information was not valid");
return  result;
    }
#[post("/delete_user", format = "json", data="<user>")]
    pub async fn delete_user(user:Json<Client>,_admin: Admin<'_>, conn:Db) -> Json<bool>{
        let user = user.into_inner();
        let delt_result = conn.run(move |temp_conn|{
            diesel::delete(client::table).filter(client::id.eq(user.id)).execute(temp_conn)
        }).await;
        return  Json(true);
        }
    
//deccription: give permission and make a recode to write a review for a user
//return: gives back confirmation that review was created
//issue: setup up parts so the code is clean or can be one statment
#[get("/reference_permission_access/<id>")]
pub async fn give_reference_access(id:i32,_admin: Admin<'_>, conn:Db) -> Json<bool>{
        let record = InsertPersonalReference{client_id: id, elucidation:"".to_string()};
        let result = conn.run(move |c| {
            diesel::insert_into(resume_reference::table)
            .values(record).get_result::<ResumeReference>(c)
        })
        .await;
    match result {
        Ok(refer) => {
            let _ = conn.run(move |c| {
                diesel::update(client::table).set(client::resume_reference_id.eq(refer.id)).execute(c)

            }).await;
            return Json(true);
        },
        Err(_) => return Json(false)               
    }
 }
    
 #[derive(Deserialize)]
 pub struct ID {pub id: i32}