CREATE TABLE address (
    user_id SERIAL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    state VARCHAR(50) NOT NULL,
    city VARCHAR(50) NOT NULL,
    street VARCHAR(255) NOT NULL,
    zip_code VARCHAR(20) NOT NULL
);
