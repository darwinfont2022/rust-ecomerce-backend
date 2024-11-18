CREATE TABLE variations (
        product_id INT NOT NULL REFERENCES products(product_id) ON DELETE CASCADE,
        variation_id SERIAL PRIMARY KEY,
        price NUMERIC(10, 2),
        available_quantity INT,
        sold_quantity INT,
        catalog_product_id VARCHAR(20)
);