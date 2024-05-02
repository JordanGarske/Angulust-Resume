use crate::{models::{reviews::{Review, InsertReview as IPF}, client_to_reference::ClientReviews}, schema::reviews as rf_Table};
use rocket::serde::json::Json;
use diesel::{RunQueryDsl, QueryDsl, SelectableHelper, ExpressionMethods};
use crate::{Db, schema::clients, models::client::Client};
pub fn routes() -> Vec<rocket::Route> {
    routes![gather_reviews,login_user]
}

//alias 
#[get("/gather_reviews")]
pub async fn gather_reviews(  conn:Db) -> Json<Vec<ClientReviews>>{
    let client_result: Vec<(Client, Review)>= conn.run(move |c|
        {  
            rf_Table::table
            .inner_join(clients::table)
            .select((Client::as_select(), Review::as_select()) )
            .load::<(Client, Review)>(c)
        }
       ).await.unwrap();
    let referecnes: Vec<ClientReviews>= client_result.into_iter()
    .map(|(cli,refen)| 
    {
        ClientReviews::new(cli, refen)
    }).collect();
    print!("{}", referecnes.len());
    Json(referecnes)
}


//this allow user to make p
//return: gives back a bool on if the message was written to the DB
#[post("/write_my_review",format = "json", data="<review>")]
pub async fn login_user( review: Json<IPF>,conn:Db) -> Json<bool>{
    let review = review.into_inner();
    print!("{}, {}", review.client_id, review.elucidation.clone());
    let result = conn.run(move |c| {
        diesel::update(rf_Table::table).filter(rf_Table::id.eq(review.client_id)).set(rf_Table::elucidation.eq(review.elucidation)).execute(c)
    })
    .await;
    match result {
        Ok(_) => Json(true),
        Err(_) => Json(false)
    }
    }