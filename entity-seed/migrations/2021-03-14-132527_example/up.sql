CREATE TABLE example(
    example_type_id BIGINT,
    status_id BIGINT,
    example_name VARCHAR(100),
    description VARCHAR(255),
    long_description TEXT,
    comments VARCHAR(255),
    example_size BIGINT,
    example_date TIMESTAMPTZ,
    another_date TIMESTAMPTZ,
    another_text VARCHAR(255),

    example_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE example_item(
    description VARCHAR(255),
    amount NUMERIC(18,3),
    amount_uom_id BIGINT,

    example_id BIGINT NOT NULL,
    example_item_seq_id BIGINT NOT NULL,
    PRIMARY KEY (example_id, example_item_seq_id)
);
CREATE TABLE example_status(
    status_end_date TIMESTAMPTZ,
    change_by_user_login_id BIGINT,
    status_id BIGINT,

    example_id BIGINT NOT NULL,
    status_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (example_id, status_date)
);
CREATE TABLE example_type(
    parent_type_id BIGINT,
    description VARCHAR(255),

    example_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE example_feature(
    feature_source_enum_id BIGINT,
    description VARCHAR(255),

    example_feature_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE example_feature_appl(
    thru_date TIMESTAMPTZ,
    example_feature_appl_type_id BIGINT,
    sequence_num BIGINT,

    example_id BIGINT NOT NULL,
    example_feature_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (example_id, example_feature_id, from_date)
);
CREATE TABLE example_feature_appl_type(
    parent_type_id BIGINT,
    description VARCHAR(255),

    example_feature_appl_type_id BIGSERIAL PRIMARY KEY
);

ALTER TABLE example ADD CONSTRAINT EXMPL_TYP
    FOREIGN KEY (example_type_id) REFERENCES example_type(example_type_id);

ALTER TABLE example_item ADD CONSTRAINT EXMPLIT_EXMP
    FOREIGN KEY (example_id) REFERENCES example(example_id);

ALTER TABLE example_status ADD CONSTRAINT EXMPLST_EXMPL
    FOREIGN KEY (example_id) REFERENCES example(example_id);

ALTER TABLE example_type ADD CONSTRAINT EXMPLTP_PAR
    FOREIGN KEY (parent_type_id) REFERENCES example_type(example_type_id);


ALTER TABLE example_feature_appl ADD CONSTRAINT EXFTAP_EXPL
    FOREIGN KEY (example_id) REFERENCES example(example_id);
ALTER TABLE example_feature_appl ADD CONSTRAINT EXFTAP_EXFT
    FOREIGN KEY (example_feature_id) REFERENCES example_feature(example_feature_id);
ALTER TABLE example_feature_appl ADD CONSTRAINT EXFTAP_TYP
    FOREIGN KEY (example_feature_appl_type_id) REFERENCES example_feature_appl_type(example_feature_appl_type_id);

ALTER TABLE example_feature_appl_type ADD CONSTRAINT EXFTAPTP_PAR
    FOREIGN KEY (parent_type_id) REFERENCES example_feature_appl_type(example_feature_appl_type_id);
