
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
