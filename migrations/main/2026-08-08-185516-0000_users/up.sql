CREATE TABLE users (
  id TEXT NOT NULL PRIMARY KEY,
  username TEXT NOT NULL,
  full_name TEXT NOT NULL,
  permissions TEXT NOT NULL DEFAULT ""
);