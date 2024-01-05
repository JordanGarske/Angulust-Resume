use crate::schema::{reviews, clients, client_to_room::client_id};
use diesel::{Queryable, Insertable, Selectable, ExpressionMethods, BoolExpressionMethods,RunQueryDsl, QueryDsl, PgConnection, SelectableHelper, associations::{Associations, Identifiable} };
use serde::{Serialize, Deserialize};
use crate::models::client::Client;
#[derive( Serialize,Queryable,Selectable, Identifiable, Associations)]
#[diesel(belongs_to(Client))]
#[diesel(table_name = reviews)]
pub struct Review {
    pub id: i32,
    pub client_id:i32,
    pub elucidation: String,
}

#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = reviews)]
pub struct InsertReview {
    pub client_id:i32,
    pub elucidation: String,
}
impl InsertReview {

    pub fn get_client_review(client: Client,conn:&mut PgConnection) -> Result<Review, diesel::result::Error>{
        reviews::table::filter( reviews::table, reviews::client_id.eq(client.id))
        .first(conn)

    }
}