use crate::schema::client;
use diesel::{Queryable, Insertable };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable,Insertable)]
#[diesel(table_name = client)]
pub struct User{
    pub client_id:i32,
    pub first_name: String,
    pub last_name: String,
    pub client_password: String,
    pub email: String,
    pub admin_privilege: bool

}
impl  User {
    fn defualt(id:i32) -> User{
        User{
            client_id: id,
            first_name: "".to_string(),
            last_name: "".to_string(),
            client_password: "".to_string(),
            email: "".to_string(),
            admin_privilege: false,
        }
    }
}

#[derive(Serialize, Queryable)]
#[diesel(table_name = client)]
pub struct UserClientInfo{
    pub first_name: String,
    pub last_name: String,
    pub email: String,

}
