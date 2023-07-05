-- Your SQL goes here

CREATE TABLE url (
	id serial PRIMARY KEY,
	short_url VARCHAR ( 255 ) NOT NULL,
	long_url VARCHAR ( 255 ) NOT NULL,
	created_on TIMESTAMP DEFAULT NOW() NOT NULL
);

CREATE TABLE analytics (
	id serial PRIMARY KEY,
	long_url VARCHAR ( 255 ) NOT NULL,
	long_url_id INT REFERENCES url(id),
	created_on TIMESTAMP DEFAULT NOW() NOT NULL
);