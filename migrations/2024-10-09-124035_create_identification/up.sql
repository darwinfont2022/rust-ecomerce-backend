CREATE TABLE identification (
                                user_id SERIAL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
                                type VARCHAR(50) NOT NULL,
                                number VARCHAR(50) NOT NULL
);
