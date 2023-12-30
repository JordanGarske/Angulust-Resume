use crate::schema::client;
use diesel::{Queryable, Insertable, Selectable, ExpressionMethods, BoolExpressionMethods,RunQueryDsl, QueryDsl, PgConnection, SelectableHelper };
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable)]
#[diesel(table_name = client)]
pub struct User {
    pub client_id: i32,
    pub personal_review_id: Option<i32>,
    pub client_password: String,
    pub email: String,
    pub admin_privilege: bool,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub profession: String,
    pub company: String,
}
impl User {

    pub fn sign_up(self,conn:&mut PgConnection) -> Result<User, diesel::result::Error>{
        diesel::insert_into(client::table)
        .values(self)
        .get_result::<User>(conn)
    }
}

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = client)]
pub struct UserLoginInfo{
    pub client_password: String,
    pub email: String,

}
impl  UserLoginInfo {
    pub fn login(self,conn:&mut PgConnection) -> Result<User, diesel::result::Error>{
        
        let client_indfication = client::client_password.eq(self.client_password)
                                .and(client::email.eq(self.email));
        client::table::filter(client::table, client_indfication).select(User::as_select()).first::<User>(conn)
    }
}

#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = client)]
pub struct AdminUser{
    pub client_id: i32,
    pub admin_privilege: bool

}
#[derive( Deserialize, Insertable)]
#[diesel(table_name = client)]
pub struct UserSignUp{
    pub client_password: String,
    pub email: String,
    pub admin_privilege: bool,
    pub first_name: String,
    pub last_name: String,
    pub phone_number: Option<String>,
    pub profession: String,
    pub company: String,
}