CREATE TABLE attribute_combinations (
    combination_id SERIAL PRIMARY KEY,
    variation_id INT NOT NULL REFERENCES variations(variation_id) ON DELETE CASCADE,
    combination_external_id VARCHAR(50),
    combination_name VARCHAR(255),
    combination_value_id VARCHAR(50),
    combination_value_name VARCHAR(255)
);