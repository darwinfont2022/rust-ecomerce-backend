CREATE TABLE alternative_phone (
    user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
    area_code VARCHAR(5),
    number VARCHAR(15),
    extension VARCHAR(10),
    PRIMARY KEY (user_id, area_code, number)
);
