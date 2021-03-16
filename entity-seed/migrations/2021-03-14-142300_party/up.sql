CREATE TABLE addendum(
    agreement_id BIGINT,
    agreement_item_seq_id BIGINT,
    addendum_creation_date TIMESTAMPTZ,
    addendum_effective_date TIMESTAMPTZ,
    addendum_text VARCHAR(255),

    addendum_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE agreement(
    product_id BIGINT,
    party_id_from BIGINT,
    party_id_to BIGINT,
    role_type_id_from BIGINT,
    role_type_id_to BIGINT,
    agreement_type_id BIGINT,
    agreement_date TIMESTAMPTZ,
    from_date TIMESTAMPTZ,
    thru_date TIMESTAMPTZ,
    description VARCHAR(255),
    text_data TEXT,

    agreement_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE agreement_attribute(
    attr_value VARCHAR(255),
    attr_description VARCHAR(255),

    agreement_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, attr_name)
);
CREATE TABLE agreement_geographical_applic(

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    geo_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, geo_id)
);
CREATE TABLE agreement_item(
    agreement_item_type_id BIGINT,
    currency_uom_id BIGINT,
    agreement_text TEXT,
    agreement_image BYTEA,

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id)
);
CREATE TABLE agreement_item_attribute(
    attr_value VARCHAR(255),
    attr_description VARCHAR(255),

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, attr_name)
);
CREATE TABLE agreement_item_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    agreement_item_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE agreement_item_type_attr(
    description VARCHAR(255),

    agreement_item_type_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (agreement_item_type_id, attr_name)
);
CREATE TABLE agreement_content(
    thru_date TIMESTAMPTZ,

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    agreement_content_type_id BIGINT NOT NULL,
    content_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (content_id, agreement_id, agreement_item_seq_id, agreement_content_type_id, from_date)
);
CREATE TABLE agreement_content_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    agreement_content_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE agreement_party_applic(

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    party_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, party_id)
);
CREATE TABLE agreement_product_appl(
    price NUMERIC(18,3),

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    product_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, product_id)
);
CREATE TABLE agreement_promo_appl(
    thru_date TIMESTAMPTZ,
    sequence_num BIGINT,

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    product_promo_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, product_promo_id, from_date)
);
CREATE TABLE agreement_facility_appl(

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    facility_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, facility_id)
);
CREATE TABLE agreement_role(

    agreement_id BIGINT NOT NULL,
    party_id BIGINT NOT NULL,
    role_type_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, party_id, role_type_id)
);
CREATE TABLE agreement_term(
    term_type_id BIGINT,
    agreement_id BIGINT,
    agreement_item_seq_id BIGINT,
    invoice_item_type_id BIGINT,
    from_date TIMESTAMPTZ,
    thru_date TIMESTAMPTZ,
    term_value NUMERIC(18,3),
    term_days BIGINT,
    text_value VARCHAR(255),
    min_quantity NUMERIC(18,3),
    max_quantity NUMERIC(18,3),
    description VARCHAR(255),

    agreement_term_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE agreement_term_attribute(
    attr_value VARCHAR(255),
    attr_description VARCHAR(255),

    agreement_term_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (agreement_term_id, attr_name)
);
CREATE TABLE agreement_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    agreement_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE agreement_type_attr(
    description VARCHAR(255),

    agreement_type_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (agreement_type_id, attr_name)
);
CREATE TABLE agreement_work_effort_applic(

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    work_effort_id BIGINT NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, work_effort_id)
);
CREATE TABLE term_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    term_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE term_type_attr(
    description VARCHAR(255),

    term_type_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (term_type_id, attr_name)
);
CREATE TABLE agreement_employment_appl(
    agreement_date TIMESTAMPTZ,
    thru_date TIMESTAMPTZ,

    agreement_id BIGINT NOT NULL,
    agreement_item_seq_id BIGINT NOT NULL,
    party_id_from BIGINT NOT NULL,
    party_id_to BIGINT NOT NULL,
    role_type_id_from BIGINT NOT NULL,
    role_type_id_to BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (agreement_id, agreement_item_seq_id, party_id_to, party_id_from, role_type_id_to, role_type_id_from, from_date)
);
CREATE TABLE comm_content_assoc_type(
    description VARCHAR(255),

    comm_content_assoc_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE comm_event_content_assoc(
    comm_content_assoc_type_id BIGINT,
    thru_date TIMESTAMPTZ,
    sequence_num BIGINT,

    content_id BIGINT NOT NULL,
    communication_event_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (content_id, communication_event_id, from_date)
);
CREATE TABLE communication_event(
    communication_event_type_id BIGINT,
    orig_comm_event_id BIGINT,
    parent_comm_event_id BIGINT,
    status_id BIGINT,
    contact_mech_type_id BIGINT,
    contact_mech_id_from BIGINT,
    contact_mech_id_to BIGINT,
    role_type_id_from BIGINT,
    role_type_id_to BIGINT,
    party_id_from BIGINT,
    party_id_to BIGINT,
    entry_date TIMESTAMPTZ,
    datetime_started TIMESTAMPTZ,
    datetime_ended TIMESTAMPTZ,
    subject VARCHAR(255),
    content_mime_type_id BIGINT,
    content TEXT,
    note VARCHAR(255),
    reason_enum_id BIGINT,
    contact_list_id BIGINT,
    header_string TEXT,
    from_string TEXT,
    to_string TEXT,
    cc_string TEXT,
    bcc_string TEXT,
    message_id VARCHAR(255),

    communication_event_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE communication_event_product(

    product_id BIGINT NOT NULL,
    communication_event_id BIGINT NOT NULL,
    PRIMARY KEY (product_id, communication_event_id)
);
CREATE TABLE communication_event_prp_typ(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    communication_event_prp_typ_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE communication_event_purpose(
    description VARCHAR(255),

    communication_event_prp_typ_id BIGINT NOT NULL,
    communication_event_id BIGINT NOT NULL,
    PRIMARY KEY (communication_event_prp_typ_id, communication_event_id)
);
CREATE TABLE communication_event_role(
    contact_mech_id BIGINT,
    status_id BIGINT,

    communication_event_id BIGINT NOT NULL,
    party_id BIGINT NOT NULL,
    role_type_id BIGINT NOT NULL,
    PRIMARY KEY (communication_event_id, party_id, role_type_id)
);
CREATE TABLE communication_event_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),
    contact_mech_type_id BIGINT,

    communication_event_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE contact_mech(
    contact_mech_type_id BIGINT,
    info_string VARCHAR(255),

    contact_mech_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE contact_mech_attribute(
    attr_value VARCHAR(255),
    attr_description VARCHAR(255),

    contact_mech_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (contact_mech_id, attr_name)
);
CREATE TABLE contact_mech_link(

    contact_mech_id_from BIGINT NOT NULL,
    contact_mech_id_to BIGINT NOT NULL,
    PRIMARY KEY (contact_mech_id_from, contact_mech_id_to)
);
CREATE TABLE contact_mech_purpose_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    contact_mech_purpose_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE contact_mech_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    contact_mech_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE contact_mech_type_attr(
    description VARCHAR(255),

    contact_mech_type_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (contact_mech_type_id, attr_name)
);
CREATE TABLE contact_mech_type_purpose(

    contact_mech_type_id BIGINT NOT NULL,
    contact_mech_purpose_type_id BIGINT NOT NULL,
    PRIMARY KEY (contact_mech_type_id, contact_mech_purpose_type_id)
);
CREATE TABLE email_address_verification(
    verify_hash VARCHAR(255),
    expire_date TIMESTAMPTZ,

    email_address BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_contact_mech(
    thru_date TIMESTAMPTZ,
    role_type_id BIGINT,
    allow_solicitation BOOLEAN,
    extension VARCHAR(255),
    verified BOOLEAN,
    comments VARCHAR(255),
    years_with_contact_mech BIGINT,
    months_with_contact_mech BIGINT,

    party_id BIGINT NOT NULL,
    contact_mech_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, contact_mech_id, from_date)
);
CREATE TABLE party_contact_mech_purpose(
    thru_date TIMESTAMPTZ,

    party_id BIGINT NOT NULL,
    contact_mech_id BIGINT NOT NULL,
    contact_mech_purpose_type_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, contact_mech_id, contact_mech_purpose_type_id, from_date)
);
CREATE TABLE postal_address(
    to_name VARCHAR(100),
    attn_name VARCHAR(100),
    address_1 VARCHAR(255),
    address_2 VARCHAR(255),
    house_number BIGINT,
    house_number_ext VARCHAR(60),
    directions VARCHAR(255),
    city VARCHAR(100),
    city_geo_id BIGINT,
    postal_code VARCHAR(60),
    postal_code_ext VARCHAR(60),
    country_geo_id BIGINT,
    state_province_geo_id BIGINT,
    county_geo_id BIGINT,
    municipality_geo_id BIGINT,
    postal_code_geo_id BIGINT,
    geo_point_id BIGINT,

    contact_mech_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE postal_address_boundary(

    contact_mech_id BIGINT NOT NULL,
    geo_id BIGINT NOT NULL,
    PRIMARY KEY (contact_mech_id, geo_id)
);
CREATE TABLE telecom_number(
    country_code VARCHAR(10),
    area_code VARCHAR(10),
    contact_number VARCHAR(60),
    ask_for_name VARCHAR(100),

    contact_mech_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE ftp_address(
    hostname VARCHAR(255),
    port BIGINT,
    username VARCHAR(255),
    ftp_password VARCHAR(255),
    binary_transfer BOOLEAN,
    file_path VARCHAR(255),
    zip_file BOOLEAN,
    passive_mode BOOLEAN,
    default_timeout BIGINT,

    contact_mech_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE valid_contact_mech_role(

    role_type_id BIGINT NOT NULL,
    contact_mech_type_id BIGINT NOT NULL,
    PRIMARY KEY (role_type_id, contact_mech_type_id)
);
CREATE TABLE need_type(
    description VARCHAR(255),

    need_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_need(
    party_type_id BIGINT,
    need_type_id BIGINT,
    communication_event_id BIGINT,
    product_id BIGINT,
    product_category_id BIGINT,
    visit_id BIGINT,
    datetime_recorded TIMESTAMPTZ,
    description VARCHAR(255),

    party_need_id BIGINT NOT NULL,
    party_id BIGINT NOT NULL,
    role_type_id BIGINT NOT NULL,
    PRIMARY KEY (party_need_id, party_id, role_type_id)
);
CREATE TABLE address_match_map(
    sequence_num BIGINT,

    map_key BIGINT NOT NULL,
    map_value BIGINT NOT NULL,
    PRIMARY KEY (map_key, map_value)
);
CREATE TABLE affiliate(
    affiliate_name VARCHAR(100),
    affiliate_description VARCHAR(255),
    year_established VARCHAR(10),
    site_type VARCHAR(255),
    site_page_views VARCHAR(255),
    site_visitors VARCHAR(255),
    date_time_created TIMESTAMPTZ,
    date_time_approved TIMESTAMPTZ,

    party_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party(
    party_type_id BIGINT,
    external_id BIGINT,
    preferred_currency_uom_id BIGINT,
    description TEXT,
    status_id BIGINT,
    created_date TIMESTAMPTZ,
    created_by_user_login BIGINT,
    last_modified_date TIMESTAMPTZ,
    last_modified_by_user_login BIGINT,
    data_source_id BIGINT,
    is_unread BOOLEAN,

    party_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_identification(
    id_value BIGINT,

    party_id BIGINT NOT NULL,
    party_identification_type_id BIGINT NOT NULL,
    PRIMARY KEY (party_id, party_identification_type_id)
);
CREATE TABLE party_identification_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    party_identification_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_geo_point(
    thru_date TIMESTAMPTZ,

    party_id BIGINT NOT NULL,
    geo_point_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, geo_point_id, from_date)
);
CREATE TABLE party_attribute(
    attr_value VARCHAR(255),
    attr_description VARCHAR(255),

    party_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (party_id, attr_name)
);
CREATE TABLE party_carrier_account(
    thru_date TIMESTAMPTZ,
    account_number BIGINT,

    party_id BIGINT NOT NULL,
    carrier_party_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, carrier_party_id, from_date)
);
CREATE TABLE party_classification(
    thru_date TIMESTAMPTZ,

    party_id BIGINT NOT NULL,
    party_classification_group_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, party_classification_group_id, from_date)
);
CREATE TABLE party_classification_group(
    party_classification_type_id BIGINT,
    parent_group_id BIGINT,
    description VARCHAR(255),

    party_classification_group_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_classification_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    party_classification_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_content(
    thru_date TIMESTAMPTZ,

    party_id BIGINT NOT NULL,
    content_id BIGINT NOT NULL,
    party_content_type_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, content_id, party_content_type_id, from_date)
);
CREATE TABLE party_content_type(
    parent_type_id BIGINT,
    description VARCHAR(255),

    party_content_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_data_source(
    visit_id BIGINT,
    comments VARCHAR(255),
    is_create BOOLEAN,

    party_id BIGINT NOT NULL,
    data_source_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, data_source_id, from_date)
);
CREATE TABLE party_group(
    group_name VARCHAR(100),
    group_name_local VARCHAR(100),
    office_site_name VARCHAR(100),
    annual_revenue NUMERIC(18,2),
    num_employees BIGINT,
    ticker_symbol VARCHAR(10),
    comments VARCHAR(255),
    logo_image_url VARCHAR(2000),

    party_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_ics_avs_override(
    avs_decline_string VARCHAR(255),

    party_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_invitation(
    party_id_from BIGINT,
    party_id BIGINT,
    to_name VARCHAR(100),
    email_address VARCHAR(255),
    status_id BIGINT,
    last_invite_date TIMESTAMPTZ,

    party_invitation_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_invitation_group_assoc(

    party_invitation_id BIGINT NOT NULL,
    party_id_to BIGINT NOT NULL,
    PRIMARY KEY (party_invitation_id, party_id_to)
);
CREATE TABLE party_invitation_role_assoc(

    party_invitation_id BIGINT NOT NULL,
    role_type_id BIGINT NOT NULL,
    PRIMARY KEY (party_invitation_id, role_type_id)
);
CREATE TABLE party_name_history(
    group_name VARCHAR(100),
    first_name VARCHAR(100),
    middle_name VARCHAR(100),
    last_name VARCHAR(100),
    personal_title VARCHAR(100),
    suffix VARCHAR(100),

    party_id BIGINT NOT NULL,
    change_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id, change_date)
);
CREATE TABLE party_note(

    party_id BIGINT NOT NULL,
    note_id BIGINT NOT NULL,
    PRIMARY KEY (party_id, note_id)
);
CREATE TABLE party_profile_default(
    default_ship_addr BIGINT,
    default_bill_addr BIGINT,
    default_pay_meth BIGINT,
    default_ship_meth BIGINT,

    party_id BIGINT NOT NULL,
    product_store_id BIGINT NOT NULL,
    PRIMARY KEY (party_id, product_store_id)
);
CREATE TABLE party_relationship(
    thru_date TIMESTAMPTZ,
    status_id BIGINT,
    relationship_name VARCHAR(100),
    security_group_id BIGINT,
    priority_type_id BIGINT,
    party_relationship_type_id BIGINT,
    permissions_enum_id BIGINT,
    position_title VARCHAR(100),
    comments VARCHAR(255),

    party_id_from BIGINT NOT NULL,
    party_id_to BIGINT NOT NULL,
    role_type_id_from BIGINT NOT NULL,
    role_type_id_to BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (party_id_from, party_id_to, role_type_id_from, role_type_id_to, from_date)
);
CREATE TABLE party_relationship_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    party_relationship_name VARCHAR(100),
    description VARCHAR(255),
    role_type_id_valid_from BIGINT,
    role_type_id_valid_to BIGINT,

    party_relationship_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_role(

    party_id BIGINT NOT NULL,
    role_type_id BIGINT NOT NULL,
    PRIMARY KEY (party_id, role_type_id)
);
CREATE TABLE party_status(
    change_by_user_login_id BIGINT,

    status_id BIGINT NOT NULL,
    party_id BIGINT NOT NULL,
    status_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (status_id, party_id, status_date)
);
CREATE TABLE party_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    party_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE party_type_attr(
    description VARCHAR(255),

    party_type_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (party_type_id, attr_name)
);
CREATE TABLE person(
    salutation VARCHAR(100),
    first_name VARCHAR(100),
    middle_name VARCHAR(100),
    last_name VARCHAR(100),
    personal_title VARCHAR(100),
    suffix VARCHAR(100),
    nickname VARCHAR(100),
    first_name_local VARCHAR(100),
    middle_name_local VARCHAR(100),
    last_name_local VARCHAR(100),
    other_local VARCHAR(100),
    member_id BIGINT,
    gender BOOLEAN,
    birth_date DATE,
    deceased_date DATE,
    height NUMERIC(18,3),
    weight NUMERIC(18,3),
    mothers_maiden_name VARCHAR(255),
    old_marital_status BOOLEAN,
    marital_status_enum_id BIGINT,
    social_security_number VARCHAR(255),
    passport_number VARCHAR(255),
    passport_expire_date DATE,
    total_years_work_experience NUMERIC(18,3),
    comments VARCHAR(255),
    employment_status_enum_id BIGINT,
    residence_status_enum_id BIGINT,
    occupation VARCHAR(100),
    years_with_employer BIGINT,
    months_with_employer BIGINT,
    existing_customer BOOLEAN,
    card_id BIGINT,

    party_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE priority_type(
    description VARCHAR(255),

    priority_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE role_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    role_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE role_type_attr(
    description VARCHAR(255),

    role_type_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (role_type_id, attr_name)
);
CREATE TABLE vendor(
    manifest_company_name VARCHAR(100),
    manifest_company_title VARCHAR(100),
    manifest_logo_url VARCHAR(2000),
    manifest_policies TEXT,

    party_id BIGSERIAL PRIMARY KEY
);

ALTER TABLE addendum ADD CONSTRAINT ADDNDM_AGRMNT
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement ADD CONSTRAINT fk_214335092397576192
    FOREIGN KEY (party_id_from) REFERENCES party(party_id);
ALTER TABLE agreement ADD CONSTRAINT fk_214335092397576193
    FOREIGN KEY (role_type_id_from) REFERENCES role_type(role_type_id);
ALTER TABLE agreement ADD CONSTRAINT AGRMNT_TYPE
    FOREIGN KEY (agreement_type_id) REFERENCES agreement_type(agreement_type_id);

ALTER TABLE agreement_attribute ADD CONSTRAINT AGRMNT_ATTR
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement_geographical_applic ADD CONSTRAINT AGRMNT_GEOAP_AGR
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement_item ADD CONSTRAINT AGRMNT_ITEM_AGR
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);
ALTER TABLE agreement_item ADD CONSTRAINT AGRMNT_ITEM_TYPE
    FOREIGN KEY (agreement_item_type_id) REFERENCES agreement_item_type(agreement_item_type_id);


ALTER TABLE agreement_item_type ADD CONSTRAINT AGRMNT_TYPEPAR
    FOREIGN KEY (parent_type_id) REFERENCES agreement_item_type(agreement_item_type_id);

ALTER TABLE agreement_item_type_attr ADD CONSTRAINT AGRMNT_ITEM_TYPATR
    FOREIGN KEY (agreement_item_type_id) REFERENCES agreement_item_type(agreement_item_type_id);

ALTER TABLE agreement_content ADD CONSTRAINT AG_CNT_PROD
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);
ALTER TABLE agreement_content ADD CONSTRAINT AG_CNT_TYPE
    FOREIGN KEY (agreement_content_type_id) REFERENCES agreement_content_type(agreement_content_type_id);

ALTER TABLE agreement_content_type ADD CONSTRAINT AGCT_TYP_PARENT
    FOREIGN KEY (parent_type_id) REFERENCES agreement_content_type(agreement_content_type_id);

ALTER TABLE agreement_party_applic ADD CONSTRAINT AGRMNT_PTYA_AGR
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);
ALTER TABLE agreement_party_applic ADD CONSTRAINT AGRMNT_PTYA_PTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE agreement_product_appl ADD CONSTRAINT fk_214335093072859136
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement_promo_appl ADD CONSTRAINT fk_214335093139968000
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement_facility_appl ADD CONSTRAINT fk_214335093198688256
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement_role ADD CONSTRAINT AGRMNT_ROLE_AGR
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);
ALTER TABLE agreement_role ADD CONSTRAINT AGRMNT_ROLE_PTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE agreement_role ADD CONSTRAINT fk_214335093265797120
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);

ALTER TABLE agreement_term ADD CONSTRAINT AGRMNT_TERM_TTYP
    FOREIGN KEY (term_type_id) REFERENCES term_type(term_type_id);
ALTER TABLE agreement_term ADD CONSTRAINT AGRMNT_TERM_AGR
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE agreement_term_attribute ADD CONSTRAINT AGRMNT_TERM_ATTR
    FOREIGN KEY (agreement_term_id) REFERENCES agreement_term(agreement_term_id);

ALTER TABLE agreement_type ADD CONSTRAINT AGRMNT_TYPE_PAR
    FOREIGN KEY (parent_type_id) REFERENCES agreement_type(agreement_type_id);

ALTER TABLE agreement_type_attr ADD CONSTRAINT AGRMNT_TYPE_ATTR
    FOREIGN KEY (agreement_type_id) REFERENCES agreement_type(agreement_type_id);

ALTER TABLE agreement_work_effort_applic ADD CONSTRAINT AGRMNT_WEA_AGRMNT
    FOREIGN KEY (agreement_id) REFERENCES agreement(agreement_id);

ALTER TABLE term_type ADD CONSTRAINT TERM_TYPE_PAR
    FOREIGN KEY (parent_type_id) REFERENCES term_type(term_type_id);

ALTER TABLE term_type_attr ADD CONSTRAINT TERM_TYPATR_TTYP
    FOREIGN KEY (term_type_id) REFERENCES term_type(term_type_id);



ALTER TABLE comm_event_content_assoc ADD CONSTRAINT COMMEV_CA_COMMEV
    FOREIGN KEY (communication_event_id) REFERENCES communication_event(communication_event_id);
ALTER TABLE comm_event_content_assoc ADD CONSTRAINT COMMEV_CA_TYP
    FOREIGN KEY (comm_content_assoc_type_id) REFERENCES comm_content_assoc_type(comm_content_assoc_type_id);

ALTER TABLE communication_event ADD CONSTRAINT COM_EVNT_TYPE
    FOREIGN KEY (communication_event_type_id) REFERENCES communication_event_type(communication_event_type_id);
ALTER TABLE communication_event ADD CONSTRAINT COM_EVNT_TPTY
    FOREIGN KEY (party_id_to) REFERENCES party(party_id);
ALTER TABLE communication_event ADD CONSTRAINT COM_EVNT_TRTYP
    FOREIGN KEY (role_type_id_to) REFERENCES role_type(role_type_id);
ALTER TABLE communication_event ADD CONSTRAINT COM_EVNT_CMTP
    FOREIGN KEY (contact_mech_type_id) REFERENCES contact_mech_type(contact_mech_type_id);
ALTER TABLE communication_event ADD CONSTRAINT COM_EVNT_FCM
    FOREIGN KEY (contact_mech_id_from) REFERENCES contact_mech(contact_mech_id);

ALTER TABLE communication_event_product ADD CONSTRAINT COMEV_PROD_CMEV
    FOREIGN KEY (communication_event_id) REFERENCES communication_event(communication_event_id);

ALTER TABLE communication_event_prp_typ ADD CONSTRAINT COM_EVNT_PRP_TYP
    FOREIGN KEY (parent_type_id) REFERENCES communication_event_prp_typ(communication_event_prp_typ_id);

ALTER TABLE communication_event_purpose ADD CONSTRAINT COM_EVNT_PRP_EVNT
    FOREIGN KEY (communication_event_id) REFERENCES communication_event(communication_event_id);
ALTER TABLE communication_event_purpose ADD CONSTRAINT COM_EVNT_PRP_TYPE
    FOREIGN KEY (communication_event_prp_typ_id) REFERENCES communication_event_prp_typ(communication_event_prp_typ_id);

ALTER TABLE communication_event_role ADD CONSTRAINT COM_EVRL_CMEV
    FOREIGN KEY (communication_event_id) REFERENCES communication_event(communication_event_id);
ALTER TABLE communication_event_role ADD CONSTRAINT COM_EVRL_PTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE communication_event_role ADD CONSTRAINT fk_214335094234681344
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);
ALTER TABLE communication_event_role ADD CONSTRAINT COM_EVRL_CMCH
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);

ALTER TABLE communication_event_type ADD CONSTRAINT COM_EVNT_TYPE_PAR
    FOREIGN KEY (parent_type_id) REFERENCES communication_event_type(communication_event_type_id);
ALTER TABLE communication_event_type ADD CONSTRAINT COM_EVNT_TYPE_CMT
    FOREIGN KEY (contact_mech_type_id) REFERENCES contact_mech_type(contact_mech_type_id);

ALTER TABLE contact_mech ADD CONSTRAINT CONT_MECH_TYPE
    FOREIGN KEY (contact_mech_type_id) REFERENCES contact_mech_type(contact_mech_type_id);

ALTER TABLE contact_mech_attribute ADD CONSTRAINT CONT_MECH_ATTR
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);

ALTER TABLE contact_mech_link ADD CONSTRAINT CONT_MECH_FCMECH
    FOREIGN KEY (contact_mech_id_from) REFERENCES contact_mech(contact_mech_id);


ALTER TABLE contact_mech_type ADD CONSTRAINT CONT_MECH_TYP_PAR
    FOREIGN KEY (parent_type_id) REFERENCES contact_mech_type(contact_mech_type_id);

ALTER TABLE contact_mech_type_attr ADD CONSTRAINT CONT_MECH_TYP_ATR
    FOREIGN KEY (contact_mech_type_id) REFERENCES contact_mech_type(contact_mech_type_id);

ALTER TABLE contact_mech_type_purpose ADD CONSTRAINT CONT_MECH_TP_TYPE
    FOREIGN KEY (contact_mech_type_id) REFERENCES contact_mech_type(contact_mech_type_id);
ALTER TABLE contact_mech_type_purpose ADD CONSTRAINT CONT_MECH_TP_PRPTP
    FOREIGN KEY (contact_mech_purpose_type_id) REFERENCES contact_mech_purpose_type(contact_mech_purpose_type_id);


ALTER TABLE party_contact_mech ADD CONSTRAINT PARTY_CMECH_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE party_contact_mech ADD CONSTRAINT fk_214335094880604160
    FOREIGN KEY (party_id) REFERENCES person(party_id);
ALTER TABLE party_contact_mech ADD CONSTRAINT fk_214335094880604161
    FOREIGN KEY (party_id) REFERENCES party_group(party_id);
ALTER TABLE party_contact_mech ADD CONSTRAINT PARTY_CMECH_ROLE
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);
ALTER TABLE party_contact_mech ADD CONSTRAINT PARTY_CMECH_CMECH
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);
ALTER TABLE party_contact_mech ADD CONSTRAINT fk_214335094884798464
    FOREIGN KEY (contact_mech_id) REFERENCES telecom_number(contact_mech_id);
ALTER TABLE party_contact_mech ADD CONSTRAINT fk_214335094884798465
    FOREIGN KEY (contact_mech_id) REFERENCES postal_address(contact_mech_id);

ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT PARTY_CMPRP_TYPE
    FOREIGN KEY (contact_mech_purpose_type_id) REFERENCES contact_mech_purpose_type(contact_mech_purpose_type_id);
ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT PARTY_CMPRP_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT fk_214335094947713024
    FOREIGN KEY (party_id) REFERENCES person(party_id);
ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT fk_214335094947713025
    FOREIGN KEY (party_id) REFERENCES party_group(party_id);
ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT PARTY_CMPRP_CMECH
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);
ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT fk_214335094947713026
    FOREIGN KEY (contact_mech_id) REFERENCES postal_address(contact_mech_id);
ALTER TABLE party_contact_mech_purpose ADD CONSTRAINT fk_214335094951907328
    FOREIGN KEY (contact_mech_id) REFERENCES telecom_number(contact_mech_id);

ALTER TABLE postal_address ADD CONSTRAINT POST_ADDR_CMECH
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);

ALTER TABLE postal_address_boundary ADD CONSTRAINT POST_ADDR_BNDRY
    FOREIGN KEY (contact_mech_id) REFERENCES postal_address(contact_mech_id);

ALTER TABLE telecom_number ADD CONSTRAINT TEL_NUM_CMECH
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);

ALTER TABLE ftp_address ADD CONSTRAINT FTP_SRV_CMECH
    FOREIGN KEY (contact_mech_id) REFERENCES contact_mech(contact_mech_id);

ALTER TABLE valid_contact_mech_role ADD CONSTRAINT VAL_CMRLE_ROLE
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);
ALTER TABLE valid_contact_mech_role ADD CONSTRAINT VAL_CMRLE_CMTYPE
    FOREIGN KEY (contact_mech_type_id) REFERENCES contact_mech_type(contact_mech_type_id);


ALTER TABLE party_need ADD CONSTRAINT PARTY_NEED_NDTP
    FOREIGN KEY (need_type_id) REFERENCES need_type(need_type_id);
ALTER TABLE party_need ADD CONSTRAINT PARTY_NEED_PTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE party_need ADD CONSTRAINT PARTY_NEED_RTYP
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);
ALTER TABLE party_need ADD CONSTRAINT PARTY_NEED_PTTP
    FOREIGN KEY (party_type_id) REFERENCES party_type(party_type_id);
ALTER TABLE party_need ADD CONSTRAINT PARTY_NEED_CMEV
    FOREIGN KEY (communication_event_id) REFERENCES communication_event(communication_event_id);


ALTER TABLE affiliate ADD CONSTRAINT AFFILIATE_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE affiliate ADD CONSTRAINT AFFILIATE_PGRP
    FOREIGN KEY (party_id) REFERENCES party_group(party_id);

ALTER TABLE party ADD CONSTRAINT PARTY_PTY_TYP
    FOREIGN KEY (party_type_id) REFERENCES party_type(party_type_id);

ALTER TABLE party_identification ADD CONSTRAINT PARTY_ID_TYPE
    FOREIGN KEY (party_identification_type_id) REFERENCES party_identification_type(party_identification_type_id);
ALTER TABLE party_identification ADD CONSTRAINT PARTY_ID_PRODUCT
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_identification_type ADD CONSTRAINT PARTY_ID_TYPE_PAR
    FOREIGN KEY (parent_type_id) REFERENCES party_identification_type(party_identification_type_id);

ALTER TABLE party_geo_point ADD CONSTRAINT PARTYGEOPT_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_attribute ADD CONSTRAINT PARTY_ATTR
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_carrier_account ADD CONSTRAINT PARTY_CRRACT_PTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_classification ADD CONSTRAINT PARTY_CLASS_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE party_classification ADD CONSTRAINT PARTY_CLASS_GRP
    FOREIGN KEY (party_classification_group_id) REFERENCES party_classification_group(party_classification_group_id);

ALTER TABLE party_classification_group ADD CONSTRAINT PARTY_CLASS_GRPPAR
    FOREIGN KEY (parent_group_id) REFERENCES party_classification_group(party_classification_group_id);
ALTER TABLE party_classification_group ADD CONSTRAINT PARTY_CLSGRP_TYPE
    FOREIGN KEY (party_classification_type_id) REFERENCES party_classification_type(party_classification_type_id);

ALTER TABLE party_classification_type ADD CONSTRAINT PARTY_CLASS_TYPPAR
    FOREIGN KEY (parent_type_id) REFERENCES party_classification_type(party_classification_type_id);

ALTER TABLE party_content ADD CONSTRAINT PARTY_CNT_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE party_content ADD CONSTRAINT PARTY_CNT_TYPE
    FOREIGN KEY (party_content_type_id) REFERENCES party_content_type(party_content_type_id);

ALTER TABLE party_content_type ADD CONSTRAINT PARTYCNT_TP_PAR
    FOREIGN KEY (parent_type_id) REFERENCES party_content_type(party_content_type_id);

ALTER TABLE party_data_source ADD CONSTRAINT PARTY_DATSRC_PTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_group ADD CONSTRAINT PARTY_GRP_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_ics_avs_override ADD CONSTRAINT PARTY_ICSAVS_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_invitation ADD CONSTRAINT PTYINV_PTY
    FOREIGN KEY (party_id_from) REFERENCES party(party_id);

ALTER TABLE party_invitation_group_assoc ADD CONSTRAINT PTYINVGA_PTYGRP
    FOREIGN KEY (party_id_to) REFERENCES party_group(party_id);
ALTER TABLE party_invitation_group_assoc ADD CONSTRAINT PTYINVGA_PTYTO
    FOREIGN KEY (party_id_to) REFERENCES party(party_id);
ALTER TABLE party_invitation_group_assoc ADD CONSTRAINT PTYINVGA_PTYINV
    FOREIGN KEY (party_invitation_id) REFERENCES party_invitation(party_invitation_id);

ALTER TABLE party_invitation_role_assoc ADD CONSTRAINT PTYINVROLE_ROLET
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);
ALTER TABLE party_invitation_role_assoc ADD CONSTRAINT PTYINVROLE_PTYINV
    FOREIGN KEY (party_invitation_id) REFERENCES party_invitation(party_invitation_id);

ALTER TABLE party_name_history ADD CONSTRAINT PTY_NMHIS_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_note ADD CONSTRAINT PARTY_NOTE_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_profile_default ADD CONSTRAINT PARTY_PROF_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_relationship ADD CONSTRAINT fk_214335096860315648
    FOREIGN KEY (party_id_from) REFERENCES party(party_id);
ALTER TABLE party_relationship ADD CONSTRAINT fk_214335096860315649
    FOREIGN KEY (role_type_id_from) REFERENCES role_type(role_type_id);
ALTER TABLE party_relationship ADD CONSTRAINT PARTY_REL_PRTYP
    FOREIGN KEY (priority_type_id) REFERENCES priority_type(priority_type_id);
ALTER TABLE party_relationship ADD CONSTRAINT PARTY_REL_TYPE
    FOREIGN KEY (party_relationship_type_id) REFERENCES party_relationship_type(party_relationship_type_id);

ALTER TABLE party_relationship_type ADD CONSTRAINT PARTY_RELTYP_PAR
    FOREIGN KEY (parent_type_id) REFERENCES party_relationship_type(party_relationship_type_id);
ALTER TABLE party_relationship_type ADD CONSTRAINT PARTY_RELTYP_VFRT
    FOREIGN KEY (role_type_id_valid_from) REFERENCES role_type(role_type_id);

ALTER TABLE party_role ADD CONSTRAINT PARTY_RLE_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
ALTER TABLE party_role ADD CONSTRAINT PARTY_RLE_ROLE
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);

ALTER TABLE party_status ADD CONSTRAINT PARTY_STS_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);

ALTER TABLE party_type ADD CONSTRAINT PARTY_TYPE_PAR
    FOREIGN KEY (parent_type_id) REFERENCES party_type(party_type_id);

ALTER TABLE party_type_attr ADD CONSTRAINT PARTY_TYP_ATTR
    FOREIGN KEY (party_type_id) REFERENCES party_type(party_type_id);

ALTER TABLE person ADD CONSTRAINT PERSON_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);


ALTER TABLE role_type ADD CONSTRAINT ROLE_TYPE_PAR
    FOREIGN KEY (parent_type_id) REFERENCES role_type(role_type_id);

ALTER TABLE role_type_attr ADD CONSTRAINT ROLE_TYPATR_RTYP
    FOREIGN KEY (role_type_id) REFERENCES role_type(role_type_id);

ALTER TABLE vendor ADD CONSTRAINT VENDOR_PARTY
    FOREIGN KEY (party_id) REFERENCES party(party_id);
