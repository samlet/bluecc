CREATE TABLE books (
    id SERIAL PRIMARY KEY,
    title varchar(256),
    author varchar(256) NOT NULL,
    status varchar(256)
);
