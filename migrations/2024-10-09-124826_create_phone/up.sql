CREATE TABLE phone (
    user_id SERIAL PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    area_code VARCHAR(5) NOT NULL,
    number VARCHAR(15) NOT NULL,
    extension VARCHAR(10),
    verified BOOLEAN NOT NULL
);
