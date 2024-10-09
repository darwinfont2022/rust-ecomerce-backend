CREATE TABLE pictures (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    picture_id VARCHAR(50),
    url TEXT,
    secure_url TEXT,
    size VARCHAR(20),
    max_size VARCHAR(20),
    quality VARCHAR(20)
);