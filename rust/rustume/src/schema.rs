// @generated automatically by Diesel CLI.

diesel::table! {
    client (id) {
        id -> Int4,
        resume_reference_id -> Nullable<Int4>,
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
    client_to_room (client_room_id) {
        client_room_id -> Int4,
        client_id -> Int4,
        room_id -> Int4,
        delete_privilege -> Bool,
        edit_privilege -> Bool,
        write_privilege -> Bool,
    }
}

diesel::table! {
    message (message_id) {
        message_id -> Int4,
        client_room_id -> Int4,
        client_id -> Int4,
        room_id -> Int4,
        #[max_length = 2000]
        client_message -> Varchar,
    }
}

diesel::table! {
    resume_reference (id) {
        id -> Int4,
        client_id -> Int4,
        #[max_length = 2000]
        elucidation -> Varchar,
    }
}

diesel::table! {
    room (id) {
        id -> Int4,
        #[max_length = 50]
        title -> Varchar,
        #[max_length = 500]
        elucidation -> Varchar,
    }
}

diesel::joinable!(client -> resume_reference (resume_reference_id));
diesel::joinable!(client_to_room -> client (client_id));
diesel::joinable!(client_to_room -> room (room_id));
diesel::joinable!(message -> client (client_id));
diesel::joinable!(message -> room (room_id));

diesel::allow_tables_to_appear_in_same_query!(
    client,
    client_to_room,
    message,
    resume_reference,
    room,
);
