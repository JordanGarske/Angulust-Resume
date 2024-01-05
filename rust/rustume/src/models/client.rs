use crate::schema::clients;
use diesel::{Queryable, Insertable, Selectable, ExpressionMethods, BoolExpressionMethods,RunQueryDsl, QueryDsl, PgConnection, SelectableHelper };
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

#[derive(Serialize, Deserialize, Queryable, Selectable)]
#[diesel(table_name = clients)]
pub struct UserLoginInfo{
    pub client_password: String,
    pub email: String,

}
impl  UserLoginInfo {
    pub fn login(self,conn:&mut PgConnection) -> Result<Client, diesel::result::Error>{
        
        let client_indfication = clients::client_password.eq(self.client_password)
                                .and(clients::email.eq(self.email));
        clients::table::filter(clients::table, client_indfication).select(Client::as_select()).first::<Client>(conn)
    }
}
//
#[derive(Serialize, Queryable, Selectable)]
#[diesel(table_name = clients)]
pub struct AdminUser{
    pub id: i32,
    pub admin_privilege: bool

}
#[derive( Deserialize, Insertable)]
#[diesel(table_name = clients)]
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