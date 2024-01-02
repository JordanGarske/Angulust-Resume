pub(crate) mod personal_info;
use self::personal_info::{gather_reviews, login_user};
//
pub fn routes() -> Vec<rocket::Route> {
    routes![gather_reviews,login_user]
}
// pub struct User_Access<'r>(&'r str);
// #[derive(Debug)]
// pub enum ApiKeyError {
//     Missing,
//     Invalid,
// }

// #[rocket::async_trait]
// impl<'r> FromRequest<'r> for Admin<'r> {
//     type Error = ApiKeyError;

//     async fn from_request(req: &'r Request<'_> ) -> Outcome<Self, Self::Error> {
        
//         let db_outcome = req.guard::<Db>().await;
//         let conn: Db = db_outcome.unwrap();
//         let id = cookie_thief(req.cookies());
//         let item:Result<AdminUser, Error>  = conn.run(move |temp_conn| {
//             //find user  information
//             client::table.find(id).select(AdminUser::as_select()).first(temp_conn)
//         })
//         .await;
//         print!("{}",id);
//         match item {
//             Ok(key) if key.admin_privilege == true => Outcome::Success(Admin("is a admin")),
//             Err(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Invalid)),
//             Ok(_) => Outcome::Error((Status::BadRequest, ApiKeyError::Missing)),
//         }
//     }

// }