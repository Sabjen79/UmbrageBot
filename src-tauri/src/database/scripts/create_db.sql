CREATE TABLE IF NOT EXISTS accounts (
    id TEXT PRIMARY KEY,
    token TEXT NOT NULL,
    name TEXT NOT NULL,
    avatar_url TEXT NOT NULL
);

CREATE TABLE IF NOT EXISTS activities (
    id TEXT PRIMARY KEY,
    bot_id TEXT NOT NULL,
    type_num INTEGER NOT NULL,
    content TEXT NOT NULL,
    url TEXT NOT NULL
);