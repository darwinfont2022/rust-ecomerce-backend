CREATE TABLE attributes (
    attribute_id SERIAL PRIMARY KEY,
    product_id INT REFERENCES products(product_id) ON DELETE CASCADE,
    attribute_name VARCHAR(255),
    value_id VARCHAR(50),
    value_name VARCHAR(255),
    attribute_group_id VARCHAR(50),
    attribute_group_name VARCHAR(255),
    value_type VARCHAR(20),
    created_ad TIMESTAMP,
    updated_ad TIMESTAMP
);
