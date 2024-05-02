// @generated automatically by Diesel CLI.

diesel::table! {
    client_to_room (id) {
        id -> Int4,
        client_id -> Int4,
        room_id -> Int4,
        delete_privilege -> Bool,
        edit_privilege -> Bool,
        write_privilege -> Bool,
    }
}

diesel::table! {
    clients (id) {
        id -> Int4,
        #[max_length = 20]
        client_password -> Varchar,
        #[max_length = 100]
        email -> Varchar,
        admin_privilege -> Bool,
        #[max_length = 50]
        first_name -> Varchar,
        #[max_length = 50]
        last_name -> Varchar,
        #[max_length = 15]
        phone_number -> Nullable<Varchar>,
        #[max_length = 50]
        profession -> Varchar,
        #[max_length = 50]
        company -> Varchar,
    }
}

diesel::table! {
    messages (id) {
        id -> Int4,
        client_id -> Int4,
        room_id -> Int4,
        #[max_length = 2000]
        cli_message -> Varchar,
    }
}

diesel::table! {
    reviews (id) {
        id -> Int4,
        client_id -> Int4,
        #[max_length = 2000]
        elucidation -> Varchar,
    }
}

diesel::table! {
    rooms (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 500]
        elucidation -> Varchar,
    }
}

diesel::joinable!(client_to_room -> clients (client_id));
diesel::joinable!(client_to_room -> rooms (room_id));
diesel::joinable!(messages -> clients (client_id));
diesel::joinable!(messages -> rooms (room_id));
diesel::joinable!(reviews -> clients (client_id));

diesel::allow_tables_to_appear_in_same_query!(
    client_to_room,
    clients,
    messages,
    reviews,
    rooms,
);
