CREATE TABLE status (
    user_id INTEGER PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    list_allow BOOLEAN NOT NULL,
    list_codes TEXT[],
    list_immediate_payment_required BOOLEAN NOT NULL,
    list_immediate_payment_reasons TEXT[],
    buy_allow BOOLEAN NOT NULL,
    buy_codes TEXT[],
    buy_immediate_payment_required BOOLEAN NOT NULL,
    buy_immediate_payment_reasons TEXT[],
    sell_allow BOOLEAN NOT NULL,
    sell_codes TEXT[],
    sell_immediate_payment_required BOOLEAN NOT NULL,
    sell_immediate_payment_reasons TEXT[],
    billing_allow BOOLEAN NOT NULL,
    billing_codes TEXT[]
);
-- Your SQL goes here
