use diesel::{QueryDsl, RunQueryDsl, SelectableHelper};
use rocket::http::CookieJar;
use rocket::{State, Shutdown};
use rocket::response::stream::{EventStream, Event};
use rocket::serde::{Serialize, Deserialize};
use rocket::tokio::sync::broadcast::{channel, Sender, error::RecvError};
use rocket::tokio::select;
use rocket::serde::json::Json;
use crate::authentications::authentication::cookie::{cookie_thief, cookie_thief_name};
use crate::models::client::Client;
use crate::Db;
use crate::models::user::message::{Messages, NewMessage, RoomMessage};
use crate::schema::{clients, messages};
pub fn routes() -> Vec<rocket::Route> {
    routes![events, post, get_room_messages ]
}

#[derive(Debug, Clone, FromForm, Serialize, Deserialize)]
#[cfg_attr(test, derive(PartialEq, UriDisplayQuery))]
#[serde(crate = "rocket::serde")]
pub struct Message {
    pub room_id: i32,
    pub client_id: i32,
    pub username: String, 
    pub message: String,
}

/// Returns an infinite stream of server-sent events. Each event is a message
/// pulled from a broadcast queue sent by the `post` handler.
#[get("/events")]
async fn events(queue: &State<Sender<Message>>,mut end: Shutdown) -> EventStream![] {
    let mut rx = queue.subscribe();
    EventStream! {
        loop {
            let msg = select! {
                msg = rx.recv() => match msg {
                    Ok(msg) => msg,
                    Err(RecvError::Closed) => break,
                    Err(RecvError::Lagged(_)) => continue,
                },
                _ = &mut end => break,
            };

            yield Event::json(&msg);
        }
    }
}

/// Receive a message from a form submission and broadcast it to any receivers.
#[post("/message", data = "<message>")]
async fn post(message: Json<Message>, conn:Db ,queue: &State<Sender<Message>>,jar: &CookieJar<'_>) {
    // A send 'fails' if there are no active subscribers. That's okay.
    let mut x = message.into_inner();
    x.client_id = cookie_thief_name(jar, "user_id").parse().expect("int should allows be stored in here");
    let mut message =x.clone();
    let result = conn.run(move |temp_conn| {
        diesel::insert_into(messages::table)
        .values(NewMessage::new(x)).get_result::<Messages>(temp_conn)
    })
    .await;
    message.username = cookie_thief_name(jar, "user_name");
    let _res = queue.send(message);
}
#[get("/messages")]
async fn get_room_messages(conn:Db ,queue: &State<Sender<Message>>,jar: &CookieJar<'_>) -> Json<Vec<RoomMessage>> {
    // A send 'fails' if there are no active subscribers. That's okay.
    let result: Vec<(Client, Messages)>= conn.run(move |c|
        {  
             messages::table
            .inner_join(clients::table)
            .select((Client::as_select(), Messages::as_select()) )
            .load::<(Client, Messages)>(c)
        }
       ).await.unwrap();
    Json(
        result.into_iter().map(|(cli,refen)|{
            RoomMessage::new(refen, cli)
        }).collect()
    )
}