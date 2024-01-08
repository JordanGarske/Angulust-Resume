pub(crate) mod personal_info;
use self::personal_info::{gather_reviews, login_user};
//
pub fn routes() -> Vec<rocket::Route> {
    routes![gather_reviews,login_user]
}
