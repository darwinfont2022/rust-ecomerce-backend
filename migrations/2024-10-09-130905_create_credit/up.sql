CREATE TABLE credit (
    user_id INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    consumed INTEGER NOT NULL,
    credit_level_id VARCHAR(50) NOT NULL
);
