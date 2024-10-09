CREATE TABLE seller_reputation (
    user_id INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    level_id VARCHAR(50),
    power_seller_status VARCHAR(50),
    transactions_period VARCHAR(50),
    transactions_total INTEGER,
    transactions_completed INTEGER,
    transactions_canceled INTEGER,
    ratings_positive INTEGER,
    ratings_negative INTEGER,
    ratings_neutral INTEGER
);
