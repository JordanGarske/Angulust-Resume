use serde::Serialize;
use crate::models::{client::Client,reviews::Review};
#[derive(Serialize)]
pub struct ClientReviews {
    pub id: i32,
    pub review_id: i32,
    pub email: String,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub profession: String,
    pub company: String,
    pub elucidation: String,
}
impl  ClientReviews{
    pub fn new(cli:Client, per_ref: Review ) -> ClientReviews{
        ClientReviews {
            id: cli.id,
            review_id: per_ref.id,
            email: cli.email,
            first_name:cli.first_name,
            last_name: cli.last_name,
            phone_number: cli.phone_number,
            profession: cli.profession,
            company: cli.company,
            elucidation: per_ref.elucidation,

        }
    }
    pub fn new_join(cli:Client, per_ref: Option<Review> ) -> ClientReviews{
        let rev = ClientReviews::review_match(per_ref);
        ClientReviews {
            id:cli.id,
            review_id: rev.id,
            email: cli.email,
            first_name:cli.first_name,
            last_name: cli.last_name,
            phone_number: cli.phone_number,
            profession: cli.profession,
            company: cli.company,
            elucidation: rev.elucidation,

        }
    }
    fn review_match(per_ref: Option<Review>) -> Review{
        match per_ref{
            Some(value) => return value,
            None => return Review{id: -1, client_id: -1, elucidation:"".to_string()}

        }
    }
}
