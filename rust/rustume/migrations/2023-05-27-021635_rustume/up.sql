CREATE TABLE client (
  client_id SERIAL PRIMARY KEY,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  client_password VARCHAR(20) UNIQUE NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  admin_privilege BOOLEAN NOT NULL
);

CREATE TABLE room (
  room_id SERIAL PRIMARY KEY,
  title VARCHAR(50) UNIQUE NOT NULL,
  elucidation VARCHAR(500) NOT NULL
);

CREATE TABLE client_to_room(
  client_room_id SERIAL PRIMARY KEY,
  client_id INT NOT NULL,
  room_id INT NOT NULL,
  delete_privilege BOOLEAN NOT NULL,
  edit_privilege BOOLEAN NOT NULL,
  write_privilege BOOLEAN NOT NULL,
  FOREIGN KEY (client_id) REFERENCES client(client_id),
  FOREIGN KEY (room_id) REFERENCES room(room_id)
);
CREATE TABLE message(
  client_room_id SERIAL PRIMARY KEY,
  client_id INT NOT NULL,
  room_id INT NOT NULL,
  client_message VARCHAR(2000) NOT NULL,
  FOREIGN KEY (client_id) REFERENCES client(client_id),
  FOREIGN KEY (room_id) REFERENCES room(room_id)
);