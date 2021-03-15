CREATE TABLE data_source(
    data_source_type_id BIGINT,
    description VARCHAR(255),

    data_source_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE data_source_type(
    description VARCHAR(255),

    data_source_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE email_template_setting(
    email_type BIGINT,
    description VARCHAR(255),
    body_screen_location VARCHAR(255),
    xslfo_attach_screen_location VARCHAR(255),
    from_address VARCHAR(320),
    cc_address VARCHAR(320),
    bcc_address VARCHAR(320),
    subject VARCHAR(255),
    content_type VARCHAR(255),

    email_template_setting_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE enumeration(
    enum_type_id BIGINT,
    enum_code VARCHAR(60),
    sequence_id BIGINT,
    description VARCHAR(255),

    enum_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE enumeration_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    enum_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE country_capital(
    country_capital_name VARCHAR(255),

    country_code BIGSERIAL PRIMARY KEY
);
CREATE TABLE country_code(
    country_abbr VARCHAR(60),
    country_number VARCHAR(60),
    country_name VARCHAR(255),

    country_code_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE country_tele_code(
    tele_code VARCHAR(60),

    country_code BIGSERIAL PRIMARY KEY
);
CREATE TABLE country_address_format(
    geo_assoc_type_id BIGINT,
    require_state_province_id BIGINT,
    require_postal_code BOOLEAN,
    postal_code_regex VARCHAR(255),
    has_postal_code_ext BOOLEAN,
    require_postal_code_ext BOOLEAN,
    address_format VARCHAR(255),

    geo_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE geo(
    geo_type_id BIGINT,
    geo_name VARCHAR(100),
    geo_code VARCHAR(60),
    geo_sec_code VARCHAR(60),
    abbreviation VARCHAR(60),
    well_known_text TEXT,

    geo_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE geo_assoc(
    geo_assoc_type_id BIGINT,

    geo_id BIGINT NOT NULL,
    geo_id_to BIGINT NOT NULL,
    PRIMARY KEY (geo_id, geo_id_to)
);
CREATE TABLE geo_assoc_type(
    description VARCHAR(255),

    geo_assoc_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE geo_point(
    geo_point_type_enum_id BIGINT,
    description VARCHAR(255),
    data_source_id BIGINT,
    latitude VARCHAR(60),
    longitude VARCHAR(60),
    elevation NUMERIC(18,6),
    elevation_uom_id BIGINT,
    information VARCHAR(255),

    geo_point_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE geo_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    geo_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE keyword_thesaurus(
    relationship_enum_id BIGINT,

    entered_keyword VARCHAR(255) NOT NULL,
    alternate_keyword VARCHAR(255) NOT NULL,
    PRIMARY KEY (entered_keyword, alternate_keyword)
);
CREATE TABLE standard_language(
    lang_code_3t VARCHAR(10),
    lang_code_3b VARCHAR(10),
    lang_code_2 VARCHAR(10),
    lang_name VARCHAR(60),
    lang_family VARCHAR(60),
    lang_charset VARCHAR(60),

    standard_language_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE custom_method(
    custom_method_type_id BIGINT,
    custom_method_name VARCHAR(255),
    description VARCHAR(255),

    custom_method_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE custom_method_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    custom_method_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE note_data(
    note_name VARCHAR(100),
    note_info TEXT,
    note_date_time TIMESTAMPTZ,

    note_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE custom_time_period(
    parent_period_id BIGINT,
    period_type_id BIGINT,
    period_num BIGINT,
    period_name VARCHAR(100),
    from_date TIMESTAMPTZ,
    thru_date TIMESTAMPTZ,
    is_closed BOOLEAN,

    custom_time_period_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE period_type(
    description VARCHAR(255),
    period_length BIGINT,
    uom_id BIGINT,

    period_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE status_item(
    status_type_id BIGINT,
    status_code VARCHAR(60),
    sequence_id BIGINT,
    description VARCHAR(255),

    status_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE status_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    status_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE status_valid_change(
    condition_expression VARCHAR(255),
    transition_name VARCHAR(100),

    status_id BIGINT NOT NULL,
    status_id_to BIGINT NOT NULL,
    PRIMARY KEY (status_id, status_id_to)
);
CREATE TABLE uom(
    uom_type_id BIGINT,
    abbreviation VARCHAR(60),
    numeric_code BIGINT,
    description VARCHAR(255),

    uom_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE uom_conversion(
    conversion_factor NUMERIC(18,3),
    custom_method_id BIGINT,
    decimal_scale BIGINT,
    rounding_mode BIGINT,

    uom_id BIGINT NOT NULL,
    uom_id_to BIGINT NOT NULL,
    PRIMARY KEY (uom_id, uom_id_to)
);
CREATE TABLE uom_conversion_dated(
    thru_date TIMESTAMPTZ,
    conversion_factor NUMERIC(18,3),
    custom_method_id BIGINT,
    decimal_scale BIGINT,
    rounding_mode BIGINT,
    purpose_enum_id BIGINT,

    uom_id BIGINT NOT NULL,
    uom_id_to BIGINT NOT NULL,
    from_date TIMESTAMPTZ NOT NULL,
    PRIMARY KEY (uom_id, uom_id_to, from_date)
);
CREATE TABLE uom_group(

    uom_group_id BIGINT NOT NULL,
    uom_id BIGINT NOT NULL,
    PRIMARY KEY (uom_group_id, uom_id)
);
CREATE TABLE uom_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    uom_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE user_preference(
    user_pref_group_type_id BIGINT,
    user_pref_value VARCHAR(255),
    user_pref_data_type BIGINT,

    user_login_id BIGINT NOT NULL,
    user_pref_type_id BIGINT NOT NULL,
    PRIMARY KEY (user_login_id, user_pref_type_id)
);
CREATE TABLE user_pref_group_type(
    description VARCHAR(255),

    user_pref_group_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE custom_screen(
    custom_screen_type_id BIGINT,
    custom_screen_name VARCHAR(255),
    custom_screen_location VARCHAR(255),
    description VARCHAR(255),

    custom_screen_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE custom_screen_type(
    parent_type_id BIGINT,
    has_table BOOLEAN,
    description VARCHAR(255),

    custom_screen_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE visual_theme_set(
    description VARCHAR(255),

    visual_theme_set_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE visual_theme(
    visual_theme_set_id BIGINT,
    description VARCHAR(255),

    visual_theme_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE visual_theme_resource(
    resource_value VARCHAR(255),

    visual_theme_id BIGINT NOT NULL,
    resource_type_enum_id BIGINT NOT NULL,
    sequence_id BIGINT NOT NULL,
    PRIMARY KEY (visual_theme_id, resource_type_enum_id, sequence_id)
);
CREATE TABLE portal_portlet(
    portlet_name VARCHAR(100),
    screen_name VARCHAR(255),
    screen_location VARCHAR(255),
    edit_form_name VARCHAR(255),
    edit_form_location VARCHAR(255),
    description VARCHAR(255),
    screenshot VARCHAR(2000),
    security_service_name VARCHAR(255),
    security_main_action VARCHAR(60),

    portal_portlet_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE portlet_category(
    description VARCHAR(255),

    portlet_category_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE portlet_portlet_category(

    portal_portlet_id BIGINT NOT NULL,
    portlet_category_id BIGINT NOT NULL,
    PRIMARY KEY (portal_portlet_id, portlet_category_id)
);
CREATE TABLE portal_page(
    portal_page_name VARCHAR(100),
    description VARCHAR(255),
    owner_user_login_id BIGINT,
    original_portal_page_id BIGINT,
    parent_portal_page_id BIGINT,
    sequence_num BIGINT,
    security_group_id BIGINT,

    portal_page_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE portal_page_column(
    column_width_pixels BIGINT,
    column_width_percentage BIGINT,

    portal_page_id BIGINT NOT NULL,
    column_seq_id BIGINT NOT NULL,
    PRIMARY KEY (portal_page_id, column_seq_id)
);
CREATE TABLE portal_page_portlet(
    column_seq_id BIGINT,
    sequence_num BIGINT,

    portal_page_id BIGINT NOT NULL,
    portal_portlet_id BIGINT NOT NULL,
    portlet_seq_id BIGINT NOT NULL,
    PRIMARY KEY (portal_page_id, portal_portlet_id, portlet_seq_id)
);
CREATE TABLE portlet_attribute(
    attr_value VARCHAR(255),
    attr_description VARCHAR(255),
    attr_type VARCHAR(255),

    portal_page_id BIGINT NOT NULL,
    portal_portlet_id BIGINT NOT NULL,
    portlet_seq_id BIGINT NOT NULL,
    attr_name BIGINT NOT NULL,
    PRIMARY KEY (portal_page_id, portal_portlet_id, portlet_seq_id, attr_name)
);
CREATE TABLE system_property(
    system_property_value VARCHAR(255),
    description VARCHAR(255),

    system_resource_id BIGINT NOT NULL,
    system_property_id BIGINT NOT NULL,
    PRIMARY KEY (system_resource_id, system_property_id)
);
CREATE TABLE telecom_method_type(
    description VARCHAR(255),

    telecom_method_type_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE telecom_gateway_config(
    description VARCHAR(255),

    telecom_gateway_config_id BIGSERIAL PRIMARY KEY
);

ALTER TABLE data_source ADD CONSTRAINT DATA_SRC_TYP
    FOREIGN KEY (data_source_type_id) REFERENCES data_source_type(data_source_type_id);


ALTER TABLE email_template_setting ADD CONSTRAINT EMAILSET_ENUM
    FOREIGN KEY (email_type) REFERENCES enumeration(enum_id);

ALTER TABLE enumeration ADD CONSTRAINT ENUM_TO_TYPE
    FOREIGN KEY (enum_type_id) REFERENCES enumeration_type(enum_type_id);

ALTER TABLE enumeration_type ADD CONSTRAINT ENUM_TYPE_PARENT
    FOREIGN KEY (parent_type_id) REFERENCES enumeration_type(enum_type_id);

ALTER TABLE country_capital ADD CONSTRAINT CNTRY_CAP_TO_CODE
    FOREIGN KEY (country_code) REFERENCES country_code(country_code_id);


ALTER TABLE country_tele_code ADD CONSTRAINT CNTRY_TELE_TO_CODE
    FOREIGN KEY (country_code) REFERENCES country_code(country_code_id);

ALTER TABLE country_address_format ADD CONSTRAINT CNY_ADR_GEO
    FOREIGN KEY (geo_id) REFERENCES geo(geo_id);
ALTER TABLE country_address_format ADD CONSTRAINT CNY_ADR_GEO_TYPE
    FOREIGN KEY (geo_assoc_type_id) REFERENCES geo_assoc_type(geo_assoc_type_id);

ALTER TABLE geo ADD CONSTRAINT GEO_TO_TYPE
    FOREIGN KEY (geo_type_id) REFERENCES geo_type(geo_type_id);

ALTER TABLE geo_assoc ADD CONSTRAINT GEO_ASSC_TO_MAIN
    FOREIGN KEY (geo_id) REFERENCES geo(geo_id);
ALTER TABLE geo_assoc ADD CONSTRAINT GEO_ASSC_TO_TYPE
    FOREIGN KEY (geo_assoc_type_id) REFERENCES geo_assoc_type(geo_assoc_type_id);


ALTER TABLE geo_point ADD CONSTRAINT GEOPOINT_DTSRC
    FOREIGN KEY (data_source_id) REFERENCES data_source(data_source_id);
ALTER TABLE geo_point ADD CONSTRAINT GEOPOINT_TYPE
    FOREIGN KEY (geo_point_type_enum_id) REFERENCES enumeration(enum_id);
ALTER TABLE geo_point ADD CONSTRAINT GPT_ELEV_UOM
    FOREIGN KEY (elevation_uom_id) REFERENCES uom(uom_id);

ALTER TABLE geo_type ADD CONSTRAINT GEO_TYPE_PARENT
    FOREIGN KEY (parent_type_id) REFERENCES geo_type(geo_type_id);

ALTER TABLE keyword_thesaurus ADD CONSTRAINT KW_THRS_RLENM
    FOREIGN KEY (relationship_enum_id) REFERENCES enumeration(enum_id);


ALTER TABLE custom_method ADD CONSTRAINT CME_TO_TYPE
    FOREIGN KEY (custom_method_type_id) REFERENCES custom_method_type(custom_method_type_id);

ALTER TABLE custom_method_type ADD CONSTRAINT CME_TYPE_PARENT
    FOREIGN KEY (parent_type_id) REFERENCES custom_method_type(custom_method_type_id);


ALTER TABLE custom_time_period ADD CONSTRAINT ORG_PRD_PARPER
    FOREIGN KEY (parent_period_id) REFERENCES custom_time_period(custom_time_period_id);
ALTER TABLE custom_time_period ADD CONSTRAINT ORG_PRD_PERTYP
    FOREIGN KEY (period_type_id) REFERENCES period_type(period_type_id);

ALTER TABLE period_type ADD CONSTRAINT PER_TYPE_UOM
    FOREIGN KEY (uom_id) REFERENCES uom(uom_id);

ALTER TABLE status_item ADD CONSTRAINT STATUS_TO_TYPE
    FOREIGN KEY (status_type_id) REFERENCES status_type(status_type_id);

ALTER TABLE status_type ADD CONSTRAINT STATUS_TYPE_PARENT
    FOREIGN KEY (parent_type_id) REFERENCES status_type(status_type_id);

ALTER TABLE status_valid_change ADD CONSTRAINT STATUS_CHG_MAIN
    FOREIGN KEY (status_id) REFERENCES status_item(status_id);

ALTER TABLE uom ADD CONSTRAINT UOM_TO_TYPE
    FOREIGN KEY (uom_type_id) REFERENCES uom_type(uom_type_id);

ALTER TABLE uom_conversion ADD CONSTRAINT UOM_CONV_MAIN
    FOREIGN KEY (uom_id) REFERENCES uom(uom_id);
ALTER TABLE uom_conversion ADD CONSTRAINT UOM_CUSTOM_METHOD
    FOREIGN KEY (custom_method_id) REFERENCES custom_method(custom_method_id);

ALTER TABLE uom_conversion_dated ADD CONSTRAINT DATE_UOM_CONV_MAIN
    FOREIGN KEY (uom_id) REFERENCES uom(uom_id);
ALTER TABLE uom_conversion_dated ADD CONSTRAINT UOMD_CUSTOM_METHOD
    FOREIGN KEY (custom_method_id) REFERENCES custom_method(custom_method_id);
ALTER TABLE uom_conversion_dated ADD CONSTRAINT UOMD_PURPOSE_ENUM
    FOREIGN KEY (purpose_enum_id) REFERENCES enumeration(enum_id);

ALTER TABLE uom_group ADD CONSTRAINT UOM_GROUP_UOM
    FOREIGN KEY (uom_id) REFERENCES uom(uom_id);

ALTER TABLE uom_type ADD CONSTRAINT UOM_TYPE_PARENT
    FOREIGN KEY (parent_type_id) REFERENCES uom_type(uom_type_id);

ALTER TABLE user_preference ADD CONSTRAINT UP_USER_GROUP_TYPE
    FOREIGN KEY (user_pref_group_type_id) REFERENCES user_pref_group_type(user_pref_group_type_id);


ALTER TABLE custom_screen ADD CONSTRAINT CSCR_TO_TYPE
    FOREIGN KEY (custom_screen_type_id) REFERENCES custom_screen_type(custom_screen_type_id);



ALTER TABLE visual_theme ADD CONSTRAINT VT_THEME_SET
    FOREIGN KEY (visual_theme_set_id) REFERENCES visual_theme_set(visual_theme_set_id);

ALTER TABLE visual_theme_resource ADD CONSTRAINT VT_RES_THEME
    FOREIGN KEY (visual_theme_id) REFERENCES visual_theme(visual_theme_id);
ALTER TABLE visual_theme_resource ADD CONSTRAINT VT_RES_TYPE_ENUM
    FOREIGN KEY (resource_type_enum_id) REFERENCES enumeration(enum_id);



ALTER TABLE portlet_portlet_category ADD CONSTRAINT PPTLTCAT_PTPL
    FOREIGN KEY (portal_portlet_id) REFERENCES portal_portlet(portal_portlet_id);
ALTER TABLE portlet_portlet_category ADD CONSTRAINT PPTLTCAT_PTLTCAT
    FOREIGN KEY (portlet_category_id) REFERENCES portlet_category(portlet_category_id);

ALTER TABLE portal_page ADD CONSTRAINT PortPage_PARENT
    FOREIGN KEY (parent_portal_page_id) REFERENCES portal_page(portal_page_id);

ALTER TABLE portal_page_column ADD CONSTRAINT PRTL_PGCOL_PAGE
    FOREIGN KEY (portal_page_id) REFERENCES portal_page(portal_page_id);

ALTER TABLE portal_page_portlet ADD CONSTRAINT PRTL_PGPTLT_PAGE
    FOREIGN KEY (portal_page_id) REFERENCES portal_page(portal_page_id);
ALTER TABLE portal_page_portlet ADD CONSTRAINT PRTL_PGPTLT_PTLT
    FOREIGN KEY (portal_portlet_id) REFERENCES portal_portlet(portal_portlet_id);

ALTER TABLE portlet_attribute ADD CONSTRAINT PTLT_ATTR_PTLT
    FOREIGN KEY (portal_portlet_id) REFERENCES portal_portlet(portal_portlet_id);



