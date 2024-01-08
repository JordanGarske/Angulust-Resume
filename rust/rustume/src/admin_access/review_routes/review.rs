use std::vec;

use crate::admin_access::admin::Admin;
use crate::models::client_to_reference::ClientReviews;
use crate::{
    models::{
        client::Client,
        reviews::{InsertReview, Review},
    },
    schema::{clients, reviews},
    Db,
};
use diesel::associations::HasTable;
use diesel::result::{Error, self};
use diesel::{ExpressionMethods, QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::serde::json::Json;
use serde::Deserialize;

//deccription: give permission and make a recode to write a review for a user
//return: gives back confirmation that review was created
#[get("/review/delete/<review_id>")]
pub async fn delete_review(review_id: i32,_admin: Admin<'_>, conn: Db) -> Json<bool> {
    let result= conn.run(move |c| {
                diesel::delete(reviews::table)
                .filter(reviews::id.eq(review_id))
                .load::<Review>(c)
        })
        .await;
    match  result {
        Ok(_) =>return Json(true),
        Err(_) =>  return Json(false),
    }
}
#[get("/review/get")]
pub async fn get_user_reviews(_admin: Admin<'_>, conn: Db) -> Json<Vec<ClientReviews>> {
    let result:Result<Vec<(Client,Option<Review>)>, Error> = conn.run(move |c| {
            clients::table
            .left_join(reviews::table)
            .select((Client::as_select(), Option::<Review>::as_select()))
            .load::<(Client,Option<Review>)>(c)
        })
        .await;
    match  result {
        Ok(value) => {
           let clients: Vec<ClientReviews> =  value.into_iter()
            .map(|(cli, rev)|{
                ClientReviews::new_JOIN(cli, rev)
            }).collect();
            Json(clients)
        },
        Err(_) =>  return Json(vec!()),
    }
}

