CREATE TABLE sale_terms (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    term_id VARCHAR(50),
    term_name VARCHAR(255),
    term_value_id VARCHAR(50),
    term_value_name VARCHAR(255)
);