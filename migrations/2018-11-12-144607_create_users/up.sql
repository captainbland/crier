CREATE TABLE crier_user (
  id SERIAL PRIMARY KEY UNIQUE,
  username VARCHAR NOT NULL UNIQUE,
  password VARCHAR NOT NULL,
  email VARCHAR NOT NULL UNIQUE
);

CREATE TABLE seller (
  id SERIAL PRIMARY KEY UNIQUE,
  crier_user_id INT4 NOT NULL REFERENCES crier_user(id),
  access_token VARCHAR NOT NULL,
  refresh_token VARCHAR NULL DEFAULT NULL,
  publishable_key VARCHAR NULL DEFAULT NULL,
  service_id VARCHAR NOT NULL,
  CONSTRAINT unique_crier_user_service_id UNIQUE (crier_user_id, service_id)
);

CREATE TABLE payer (
  id SERIAL PRIMARY KEY UNIQUE,
  crier_user_id INT4 NOT NULL REFERENCES crier_user(id),
  service_customer_id VARCHAR DEFAULT NULL UNIQUE,
  service_payment_source VARCHAR NOT NULL UNIQUE
);

CREATE TABLE listing (
  id SERIAL PRIMARY KEY UNIQUE,
  seller_id INT4 NOT NULL REFERENCES seller(id),
  price INT4 NOT NULL,
  currency VARCHAR(3) NOT NULL,
  amount INT4 DEFAULT NULL,
  limited_amount BOOLEAN DEFAULT FALSE
);