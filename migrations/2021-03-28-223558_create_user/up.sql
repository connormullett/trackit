CREATE TABLE app_user (
  id SERIAL PRIMARY KEY,
  username VARCHAR(64) NOT NULL,
  password_hash VARCHAR(32) NOT NULL
)