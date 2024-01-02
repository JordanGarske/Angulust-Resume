use crate::{models::{personal_reference::{ResumeReference, InsertPersonalReference as IPF}, client_to_reference::ClientReference}, schema::resume_reference as rf_Table};
use crate::models::client_to_reference;
use rocket::{serde::json::Json, http::CookieJar};
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use serde::Serialize;
use crate::{Db, schema::client, models::client::Client};
//alias 
#[get("/gather_reviews")]
pub async fn gather_reviews(  conn:Db) -> Json<Vec<ClientReference>>{
    let client_result: Vec<(Client, ResumeReference)>= conn.run(move |c|
        {  
            rf_Table::table
            .inner_join(client::table)
            .select((Client::as_select(), ResumeReference::as_select()) )
            .load::<(Client, ResumeReference)>(c)
        }
       ).await.unwrap();
    let referecnes= client_result.iter()
    .map(|(cli,refen)| 
    {
         ClientReference::clone_new(cli, refen)
    }).collect();
    Json(referecnes)
}



#[post("/write_my_review",format = "json", data="<review>")]
pub async fn login_user( review: Json<IPF>,conn:Db) -> Json<bool>{
    let review = review.into_inner();
    let result = conn.run(move |c| {
        diesel::update(rf_Table::table).filter(rf_Table::client_id.eq(review.client_id)).set(rf_Table::elucidation.eq(review.elucidation)).execute(c)
    })
    .await;
    match result {
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
    }
    
