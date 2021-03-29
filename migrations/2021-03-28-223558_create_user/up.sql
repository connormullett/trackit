CREATE TABLE app_user (
  id SERIAL PRIMARY KEY,
  username VARCHAR(32) NOT NULL,
  password_hash VARCHAR(128) NOT NULL
)