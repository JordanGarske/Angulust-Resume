use rocket::http::{CookieJar, Cookie};

pub fn bake_cookie(jar: &CookieJar<'_>, key: String  , id : String ){
    let cookie = Cookie::build(key, id.to_string()).http_only(true).secure(true);
    jar.add_private(cookie.finish());
}
pub fn cookie_theif(jar: &CookieJar<'_>) -> i32{
    match jar.get_private("user_id"){
        Some(value) =>
            match value.value().parse::<i32>(){
                Ok(num)=> return num,
                Err(_) => return  -1
            },
        None=> return -1
    }
}