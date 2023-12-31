use crate::models::client::Client;
use crate::schema::resume_reference;
use diesel::{Queryable, Insertable, Selectable, Identifiable, associations::Associations};
use serde::{Serialize, Deserialize};
#[derive( Queryable,Selectable, Identifiable, Associations)]
#[diesel(table_name = resume_reference)]
#[diesel(belongs_to(Client))]
pub struct ResumeReference {
    pub id: i32,
    pub client_id: i32,
    pub elucidation: String,
}

#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = resume_reference)]
pub struct InsertPersonalReference {
    pub client_id: i32,
    pub elucidation: String,
}