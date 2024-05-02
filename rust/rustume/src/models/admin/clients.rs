use crate::schema::clients;
use diesel::{Queryable, Insertable, Selectable, RunQueryDsl, PgConnection };
use serde::{Serialize, Deserialize};
#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = clients)]
pub struct Client {
    pub id: i32,
    pub client_password: String,
    pub email: String,
    pub admin_privilege: bool,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub profession: String,
    pub company: String,
}
impl Client {

    pub fn sign_up(self,conn:&mut PgConnection) -> Result<Client, diesel::result::Error>{
        diesel::insert_into(clients::table)
        .values(self)
        .get_result::<Client>(conn)
    }
}