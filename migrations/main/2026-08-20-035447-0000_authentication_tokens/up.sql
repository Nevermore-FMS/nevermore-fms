CREATE TABLE authentication_tokens (
  id TEXT NOT NULL PRIMARY KEY,
  target_type TEXT NOT NULL,
  target_id TEXT NOT NULL,
  token_hash TEXT NOT NULL,
  created_timestamp INTEGER NOT NULL,
  expires_at_timestamp INTEGER NOT NULL
);