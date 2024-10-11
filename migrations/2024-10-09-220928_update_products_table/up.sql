CREATE TABLE product_media (
    product_id INT NOT NULL,
    permalink VARCHAR NOT NULL,
    thumbnail_id VARCHAR NOT NULL,
    thumbnail VARCHAR NOT NULL,
    PRIMARY KEY (product_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id) ON DELETE CASCADE
);

CREATE TABLE product_price(
    product_id INT NOT NULL,
    price NUMERIC(10, 2),
    base_price NUMERIC(10, 2),
    original_price NUMERIC(10, 2),
    currency_id VARCHAR(3),
    PRIMARY KEY (product_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id) ON DELETE CASCADE
);

CREATE TABLE product_inventory(
    product_id INT NOT NULL,
    initial_quantity INT,
    available_quantity INT,
    sold_quantity INT,
    PRIMARY KEY (product_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id) ON DELETE CASCADE
);

CREATE TABLE product_listing(
    product_id INT NOT NULL,
    listing_type_id VARCHAR(20),
    listing_source VARCHAR(50),
    catalog_listing BOOLEAN,
    PRIMARY KEY (product_id),
    FOREIGN KEY (product_id) REFERENCES products(product_id) ON DELETE CASCADE
);

ALTER TABLE products
    DROP COLUMN permalink,
    DROP COLUMN thumbnail_id,
    DROP COLUMN thumbnail,
    DROP COLUMN price,
    DROP COLUMN base_price,
    DROP COLUMN original_price,
    DROP COLUMN currency_id,
    DROP COLUMN listing_type_id,
    DROP COLUMN listing_source,
    DROP COLUMN catalog_listing;