# Angulust-Resume
db                  | 2023-12-19 05:38:01.941 UTC [112] ERROR:  relation "client" does not exist at character 13
db                  | 2023-12-19 05:38:01.941 UTC [112] STATEMENT:  INSERT INTO "client" ("client_id", "first_name", "last_name", "client_password", "email", "admin_privilege") VALUES ($1, $2, $3, $4, $5, $6) RETURNING "client"."client_id", "client"."first_name", "client"."last_name", "client"."client_password", "client"."email", "client"."admin_privilege"
rustume             | thread 'rocket-worker-thread' panicked at 'the information was not valid: DatabaseError(Unknown, "relation \"client\" does not exist")', src/authentication/signup.rs:23:6
rustume             | note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
rustume             |    >> Handler create_user panicked.
rustume             |    >> A panic is treated as an internal server error.
rustume             |    >> No 500 catcher registered. Using Rocket default.
pgadmin4_container  | 2023-12-19 05:38:06,342: ERROR    pgadmin:        400 Bad Request: The CSRF session token is missing.