use crate::admin_access::admin::Admin;
use crate::{
    models::{
        client::Client,
        reviews::{InsertReview, Review},
    },
    schema::{clients, reviews},
    Db,
};
use diesel::{ExpressionMethods, RunQueryDsl};
use rocket::serde::json::Json;
use serde::Deserialize;
//deccription: give permission and make a recode to write a review for a user
//return: gives back confirmation that review was created

#[post("/client/delete", format = "json", data = "<user>")]
pub async fn delete_user(user: Json<Client>, _admin: Admin<'_>, conn: Db) -> Json<&str> {

    let user = user.into_inner();
    let delt_result = conn
    .run(move |temp_conn| {
        diesel::delete(reviews::table)
            .filter(reviews::client_id.eq(user.id))
            .execute(temp_conn)
    })
    .await;
    let delt_result = conn
        .run(move |temp_conn| {
            diesel::delete(clients::table)
                .filter(clients::id.eq(user.id))
                .execute(temp_conn)
        })
        .await;
    return Json("worked");
}
#[get("/client/get")]
pub async fn get_clients(_admin: Admin<'_>, conn: Db) -> Json<Vec<Client>> {
    let result = conn
        .run(move |c| {
             clients::table
             .get_results::<Client>(c)
        })
        .await;
    match result {
        Ok(users) => return Json(users),
        Err(_) => return Json(vec!()),
    }
}

//deccription: give permission and make a recode to write a review for a user
//return: gives back confirmation that review was created
//issue: setup up parts so the code is clean or can be one statment
#[get("/reference_permission_access/<id>")]
pub async fn give_reference_access(id: i32, _admin: Admin<'_>, conn: Db) -> Json<(bool, Review)> {
    let record = InsertReview {
        client_id: id,
        elucidation: "".to_string(),
    };
    let result = conn
        .run(move |c| {
            diesel::insert_into(reviews::table)
                .values(record)
                .get_result::<Review>(c)
        })
        .await;
    match result {
        Ok(refer) => return Json((true, refer)),
        Err(_) => return Json((false, Review{client_id:-1, id:-1, elucidation: "".to_string()})),
    }
}

#[derive(Deserialize)]
pub struct ID {
    pub id: i32,
}
