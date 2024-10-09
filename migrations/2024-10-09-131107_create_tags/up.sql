CREATE TABLE tags (
    id SERIAL PRIMARY KEY,
    name VARCHAR(50) NOT NULL UNIQUE
);

CREATE TABLE user_tags (
     user_id INTEGER REFERENCES users(id) ON DELETE CASCADE,
     tag_id INT REFERENCES tags(id) ON DELETE CASCADE,
     PRIMARY KEY (user_id, tag_id)
);
