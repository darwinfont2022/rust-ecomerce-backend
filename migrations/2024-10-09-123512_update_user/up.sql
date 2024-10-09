CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       nickname VARCHAR(255) NOT NULL,
                       registration_date TIMESTAMP WITH TIME ZONE NOT NULL,
                       first_name VARCHAR(255) NOT NULL,
                       last_name VARCHAR(255) NOT NULL,
                       country_id CHAR(2) NOT NULL,
                       email VARCHAR(255) NOT NULL UNIQUE,
                       user_type VARCHAR(50) NOT NULL,
                       points INTEGER NOT NULL,
                       site_id VARCHAR(50) NOT NULL,
                       permalink VARCHAR(255) NOT NULL,
                       seller_experience VARCHAR(50),
                       status_site_status VARCHAR(50) NOT NULL,
                       pago_tc_accepted BOOLEAN NOT NULL,
                       pago_account_type VARCHAR(50),
                       delivery VARCHAR(50),
                       immediate_payment BOOLEAN NOT NULL,
                       confirmed_email BOOLEAN NOT NULL,
                       required_action VARCHAR(255)
);
-- Your SQL goes here
