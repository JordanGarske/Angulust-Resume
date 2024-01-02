use serde::Serialize;
use crate::models::{client::Client,personal_reference::ResumeReference};

#[derive(Serialize)]
pub struct ClientReference {
    pub id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub profession: String,
    pub company: String,
    pub elucidation: String,
}
impl  ClientReference{
    pub fn clone_new(cli:&Client, per_ref: &ResumeReference ) -> ClientReference{
        ClientReference {
            id:per_ref.id,
            email: cli.email.to_string(),
            first_name:cli.first_name.to_string(),
            last_name: cli.last_name.to_string(),
            phone_number: cli.phone_number.clone(),
            profession: cli.profession.to_string(),
            company: cli.company.to_string(),
            elucidation: per_ref.elucidation.to_string(),

        }
    }
}