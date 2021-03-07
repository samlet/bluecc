CREATE TABLE products (
	product_no INTEGER,
	description TEXT,
	product_cost NUMERIC
);

ALTER TABLE products ADD PRIMARY KEY (product_no);
-- ALTER TABLE products ADD COLUMN product_no SERIAL PRIMARY KEY;

CREATE TABLE example(
    example_type_id VARCHAR(20),
    status_id VARCHAR(20),
    example_name VARCHAR(100),
    description VARCHAR(255),
    long_description TEXT,
    comments VARCHAR(255),
    example_size NUMERIC(20,0),
    example_date TIMESTAMPTZ,
    another_date TIMESTAMPTZ,
    another_text VARCHAR(255),

    example_id SERIAL PRIMARY KEY
);

CREATE TABLE example_item(
    description VARCHAR(255),
    amount FLOAT8,
    amount_uom_id VARCHAR(20),

    example_id VARCHAR(20),
    example_item_seq_id VARCHAR(20),
    PRIMARY KEY (example_id, example_item_seq_id)
);

-- mnemonic
CREATE TABLE mnemonics (
	id SERIAL PRIMARY KEY,
	path VARCHAR(255) NOT NULL,
	num_value INTEGER NOT NULL,
	string_value VARCHAR(255) NOT NULL,
	description TEXT
);

CREATE INDEX index_mnemonic_path ON mnemonics (path);
CREATE INDEX index_mnemonic_num ON mnemonics (num_value);
