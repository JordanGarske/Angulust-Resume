use crate::schema::{clients, rooms};
use diesel::{Queryable, Insertable, Selectable, ExpressionMethods, BoolExpressionMethods,RunQueryDsl, QueryDsl, PgConnection, SelectableHelper };
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = rooms)]
pub struct Room<'a> {
    id:i32,
    title: &'a str,
    elucidation: &'a str,
}