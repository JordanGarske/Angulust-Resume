
#[cfg(test)]
mod user_test {

    use crate::models::credential::CredentialApproval;
    use crate::models::client;
    //rocket crates
    //rustume crates
    use crate::rocket;
    use rocket::{http::ContentType, local::blocking::Client};    
    use rocket::http::Status;
    #[test]
    fn login_as_admin() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let  response = post_admin("/login", &client);
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let  data: CredentialApproval  = response.into_json().expect("login in user");
        assert_eq!(data.admin, true);
    }
    #[test]
    fn give_admin_review_access() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let  _response = post_admin("/login", &client);
        let admin_response = get_admin("/admin/client/get", &client);
        assert_eq!(admin_response.status(), Status::Ok);
        assert_eq!(admin_response.content_type(), Some(ContentType::JSON));
        let  _data: Vec<client::Client>  = admin_response.into_json().expect("login in user");
        assert_eq!(true, true);

        // assert_eq!(response.content_type(), ContentType::);
    }
    fn post_admin<'a>(url: &'a str, client: &'a Client) -> rocket::local::blocking::LocalResponse<'a>{
         client.post(url)
        .header(ContentType::JSON)
        .body(r##"{
            "client_password": "j",
            "email": "j"
        }"##)
        .dispatch()
    
    }
    fn get_admin<'a>(url: &'a str, client: &'a Client) -> rocket::local::blocking::LocalResponse<'a>{
        client.get(url)
       .dispatch()
   
   }
}    