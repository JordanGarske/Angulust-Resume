use rocket::http::{CookieJar, Cookie};

pub fn bake_cookie(jar: &CookieJar<'_>, key: String  , id : String ){
    let cookie = Cookie::build((key, id.to_string()))
                .http_only(true)
                .secure(true)
                .build();
    jar.add_private(cookie);
}
pub fn cookie_thief(jar: &CookieJar<'_>, key: &str) -> i32{
    match jar.get_private(key){
        Some(value) =>
            match value.value().parse::<i32>(){
                Ok(num)=> return num,
                Err(_) => return  -1
            },
        None=> return -1
    }
}

pub fn cookie_thief_name(jar: &CookieJar<'_>, key: &str) -> String {
    match jar.get_private(key){
        Some(value) => value.value().to_string(),
        None=> return  "".to_string()
    }
}