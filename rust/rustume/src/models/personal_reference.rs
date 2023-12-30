use crate::schema::{client, personal_reference};
use diesel::{Queryable, Insertable, Selectable, ExpressionMethods, BoolExpressionMethods,RunQueryDsl, QueryDsl, PgConnection, SelectableHelper };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = personal_reference)]
pub struct Personal_Reference {
    pub personal_review_id: i32,
    pub client_id: i32,
    pub elucidation: String,
}
#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = personal_reference)]
pub struct Insert__Personal_Reference {
    pub client_id: i32,
    pub elucidation: String,
}
