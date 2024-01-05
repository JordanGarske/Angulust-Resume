-- Tables for chat rooms
CREATE TABLE rooms (
  id SERIAL PRIMARY KEY,
  title VARCHAR(50) UNIQUE NOT NULL,
  elucidation VARCHAR(500) NOT NULL
);


-- Table for clients
CREATE TABLE clients (
  id SERIAL PRIMARY KEY,
  client_password VARCHAR(20) UNIQUE NOT NULL,
  email VARCHAR(100) UNIQUE NOT NULL,
  admin_privilege BOOLEAN NOT NULL,
  first_name VARCHAR(50) NOT NULL,
  last_name VARCHAR(50) NOT NULL,
  phone_number VARCHAR(15),
  profession VARCHAR(50) NOT NULL,
  company VARCHAR(50) NOT NULL
);
-- Table for resume references
CREATE TABLE reviews (
  id SERIAL PRIMARY KEY,
  client_id INT NOT NULL,
  elucidation VARCHAR(2000) NOT NULL,
  FOREIGN KEY (client_id) REFERENCES clients(id)
);

CREATE TABLE client_to_room (
  id SERIAL PRIMARY KEY,
  client_id INT NOT NULL,
  room_id INT NOT NULL,
  delete_privilege BOOLEAN NOT NULL,
  edit_privilege BOOLEAN NOT NULL,
  write_privilege BOOLEAN NOT NULL,
  FOREIGN KEY (client_id) REFERENCES clients(id),
  FOREIGN KEY (room_id) REFERENCES rooms(id)
);

CREATE TABLE messages (
  id SERIAL PRIMARY KEY,
  client_room_id INT NOT NULL,
  client_id INT NOT NULL,
  room_id INT NOT NULL,
  cli_message VARCHAR(2000) NOT NULL,
  FOREIGN KEY (client_id) REFERENCES clients(id),
  FOREIGN KEY (room_id) REFERENCES rooms(id)
);
