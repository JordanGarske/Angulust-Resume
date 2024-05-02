
#[cfg(test)]
mod test {
    use crate::authentications::authentication::cookie::cookie_thief;
    use crate::authentications::authentication::{login, signup};
    use crate::models::credential::CredentialApproval;
    use crate::rocket;
    use rocket::local::blocking::Client;
    use rocket::http::{Status, ContentType};
    //rocket crates

    //rustume crates
    
    #[test]
    fn login_test() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!(login::login_user))
            .header(ContentType::JSON)
            .body(r##"{
                "client_password": "j",
                "email": "j"
            }"##)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        assert_eq!(cookie_thief(response.cookies(), "user_id"), 1);
        let response_body:CredentialApproval  = response.into_json().expect("login in user");
        assert_eq!(response_body.approved, true);
        assert_eq!(response_body.approved, true);
        // assert_eq!(response.content_type(), ContentType::);
    }
    #[test]
    fn sign_up_test() {
        let client = Client::tracked(rocket()).expect("valid rocket instance");
        let response = client.post(uri!(signup::create_user))
            .header(ContentType::JSON)
            .body(r##"{
                "client_password": "password123",
                "email": "john.doe@gmail.com",
                "admin_privilege": false,
                "first_name": "John",
                "last_name": "Doe",
                "phone_number": null,
                "profession": "Engineer",
                "company": "Example Company"
            }"##)
            .dispatch();
        assert_eq!(response.status(), Status::Ok);
        assert_eq!(response.content_type(), Some(ContentType::JSON));
        let response_body:bool  = response.into_json().unwrap();
        assert_eq!(response_body, true);
    }

        #[test]
        fn sign_up_revert_test() {
            let client = Client::tracked(rocket()).expect("valid rocket instance");
            let response = client.post(uri!(login::login_user))
                .header(ContentType::JSON)
                .body(r##"{
                    "client_password": "j",
                    "email": "j"
                }"##)
                .dispatch();
            assert_eq!(response.status(), Status::Ok);

            let _response_two = client.post("/admin/client/delete")             
            .header(ContentType::JSON)
            .body(r##"{
                "client_password": "password123",
                "email": "john.doe@gmail.com",
                "admin_privilege": false,
                "first_name": "John",
                "last_name": "Doe",
                "phone_number": null,
                "profession": "Engineer",
                "company": "Example Company"
            }"##)
            .dispatch();
        // let response_body:String  = response_two.into_json().expect("login in user");
        // assert_eq!(response_body, "worked");
    
        }

}    