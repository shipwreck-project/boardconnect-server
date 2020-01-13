CREATE TABLE users
(
  user_key SERIAL PRIMARY KEY,
  name VARCHAR(25) NOT NULL,
  description TINYTEXT NOT NULL,
  created_at DATETIME NOT NULL,
);