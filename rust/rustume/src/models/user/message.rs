use crate::{schema::messages, chat::room::Message};
use diesel::{Queryable, Insertable, Selectable, associations::Associations };
use serde::{Serialize, Deserialize};
use crate::models::client::Client;
#[derive(Serialize, Deserialize, Queryable,Insertable,Selectable,Associations)]
#[diesel(belongs_to(Client))]
#[diesel(table_name = messages)]
pub struct Messages {
    pub id: i32,
    pub client_id: i32,
    pub room_id: i32,
    pub cli_message: String,
}

#[derive(Insertable)]
#[diesel(table_name = messages)]
pub struct NewMessage {
    pub client_id: i32,
    pub room_id: i32,
    pub cli_message: String,
}

impl  NewMessage {
    pub fn new(mess: Message) -> NewMessage{
        NewMessage{
            client_id: mess.client_id,
            room_id: mess.room_id,
            cli_message: mess.message,
        }
    }
}
#[derive( Serialize)]
pub struct RoomMessage{
    pub id : i32,
    pub client_id: i32,
    pub room_id: i32,
    pub username: String , 
    pub message: String,
}
impl RoomMessage{
    pub fn new(mess: Messages, client: Client) -> RoomMessage{
        let user_name = client.first_name + &client.last_name;
        RoomMessage{
            id: 1,
            client_id: mess.client_id,
            room_id: mess.room_id,
            username: user_name,
            message: mess.cli_message,
        }
    }
}