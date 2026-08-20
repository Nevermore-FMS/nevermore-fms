CREATE TABLE user_credentials (
  id TEXT NOT NULL PRIMARY KEY,
  user_id TEXT NOT NULL,
  credential_type TEXT NOT NULL,
  credential_body TEXT NOT NULL,
  created_timestamp INTEGER NOT NULL,

  FOREIGN KEY (user_id) REFERENCES users(id)
);