CREATE DATABASE circles
(
  circle_key SERIAL NOT NULL
  FOREIGN KEY (owner) REFERENCES users(user_key)
  description TINYTEXT NOT NULL,
  created_at DATETIME NOT NULL,
)