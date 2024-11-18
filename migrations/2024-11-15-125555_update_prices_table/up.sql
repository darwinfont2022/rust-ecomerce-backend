ALTER TABLE variations
    DROP COLUMN price;

CREATE TABLE product_price(
    price_id SERIAL PRIMARY KEY,
    product_id INT NOT NULL,
    price NUMERIC(10, 2),
    base_price NUMERIC(10, 2),
    original_price NUMERIC(10, 2),
    currency_id VARCHAR(3),
    price_type VARCHAR(250),
    create_at TIMESTAMP,
    update_at TIMESTAMP,
    FOREIGN KEY (product_id) REFERENCES products(product_id) ON DELETE CASCADE
);

CREATE TABLE variation_price(
    price_id SERIAL PRIMARY KEY,
    variation_id INT NOT NULL,
    price NUMERIC(10, 2),
    base_price NUMERIC(10, 2),
    original_price NUMERIC(10, 2),
    currency_id VARCHAR(3),
    price_type VARCHAR(250),
    create_at TIMESTAMP,
    update_at TIMESTAMP,
    FOREIGN KEY (variation_id) REFERENCES variations(variation_id) ON DELETE CASCADE
);