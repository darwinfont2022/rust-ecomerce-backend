-- Tabla de descripciones
CREATE TABLE descriptions (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    description_id VARCHAR(50)
);

-- Tabla de atributos
CREATE TABLE attributes (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    attribute_id VARCHAR(50),
    attribute_name VARCHAR(255),
    value_id VARCHAR(50),
    value_name VARCHAR(255),
    attribute_group_id VARCHAR(50),
    attribute_group_name VARCHAR(255),
    value_type VARCHAR(20)
);

-- Tabla de variaciones
CREATE TABLE variations (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    variation_id BIGINT,
    price NUMERIC(10, 2),
    available_quantity INT,
    sold_quantity INT,
    catalog_product_id VARCHAR(20)
);

-- Tabla de combinaciones de atributos para variaciones
CREATE TABLE attribute_combinations (
    product_id INT REFERENCES variations(product_id) ON DELETE CASCADE,
    combination_id VARCHAR(50),
    combination_name VARCHAR(255),
    combination_value_id VARCHAR(50),
    combination_value_name VARCHAR(255),
    PRIMARY KEY (product_id, combination_id)  -- Clave compuesta si hay múltiples combinaciones
);

-- Tabla de métodos de pago del vendedor
CREATE TABLE seller_payment_methods (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    method_id VARCHAR(50),
    method_name VARCHAR(255)
);

-- Tabla de envíos
CREATE TABLE shipping (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    mode VARCHAR(20),
    free_shipping BOOLEAN,
    logistic_type VARCHAR(20)
);

-- Tabla de detalles de la dirección del vendedor
CREATE TABLE seller_address (
    product_id INT PRIMARY KEY REFERENCES products(product_id) ON DELETE CASCADE,
    city_id VARCHAR(50),
    city_name VARCHAR(255),
    state_id VARCHAR(50),
    state_name VARCHAR(255),
    country_id VARCHAR(50),
    country_name VARCHAR(255),
    neighborhood_id VARCHAR(50),
    neighborhood_name VARCHAR(255)
);