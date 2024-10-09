CREATE TABLE buyer_reputation (
    user_id INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    canceled_transactions INTEGER,
    transactions_period VARCHAR(50),
    transactions_total INTEGER,
    transactions_completed INTEGER,
    transactions_canceled_total INTEGER,
    transactions_canceled_paid INTEGER,
    transactions_unrated_total INTEGER,
    transactions_unrated_paid INTEGER,
    transactions_not_yet_rated_total INTEGER,
    transactions_not_yet_rated_paid INTEGER,
    transactions_not_yet_rated_units INTEGER
);
