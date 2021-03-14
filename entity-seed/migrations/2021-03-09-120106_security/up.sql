CREATE TABLE x509_issuer_provision(
    common_name VARCHAR(255),
    organizational_unit VARCHAR(255),
    organization_name VARCHAR(255),
    city_locality VARCHAR(255),
    state_province VARCHAR(255),
    country VARCHAR(255),
    serial_number VARCHAR(255),

    cert_provision_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE user_login(
    current_password VARCHAR(255),
    password_hint VARCHAR(255),
    is_system CHAR(1),
    enabled CHAR(1),
    has_logged_out CHAR(1),
    require_password_change CHAR(1),
    last_currency_uom BIGINT NOT NULL,
    last_locale VARCHAR(10),
    last_time_zone BIGINT NOT NULL,
    disabled_date_time TIMESTAMPTZ,
    successive_failed_logins NUMERIC(20,0),
    external_auth_id BIGINT NOT NULL,
    user_ldap_dn BIGINT NOT NULL,
    disabled_by BIGINT NOT NULL,

    user_login_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE user_login_password_history(
    thru_date TIMESTAMPTZ,
    current_password VARCHAR(255),

    user_login_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ,
    PRIMARY KEY (user_login_id, from_date)
);
CREATE TABLE user_login_history(
    visit_id BIGINT NOT NULL,
    thru_date TIMESTAMPTZ,
    password_used VARCHAR(255),
    successful_login CHAR(1),
    origin_user_login_id BIGINT NOT NULL,

    user_login_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ,
    PRIMARY KEY (user_login_id, from_date)
);
CREATE TABLE user_login_session(
    saved_date TIMESTAMPTZ,
    session_data TEXT,

    user_login_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE security_group(
    group_name VARCHAR(255),
    description VARCHAR(255),

    group_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE security_group_permission(
    thru_date TIMESTAMPTZ,

    group_id BIGINT NOT NULL,
    permission_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ,
    PRIMARY KEY (group_id, permission_id, from_date)
);
CREATE TABLE security_permission(
    description VARCHAR(255),

    permission_id BIGSERIAL PRIMARY KEY
);
CREATE TABLE user_login_security_group(
    thru_date TIMESTAMPTZ,

    user_login_id BIGINT NOT NULL,
    group_id BIGINT NOT NULL,
    from_date TIMESTAMPTZ,
    PRIMARY KEY (user_login_id, group_id, from_date)
);
CREATE TABLE protected_view(
    max_hits NUMERIC(20,0),
    max_hits_duration NUMERIC(20,0),
    tarpit_duration NUMERIC(20,0),

    group_id BIGINT NOT NULL,
    view_name_id BIGINT NOT NULL,
    PRIMARY KEY (group_id, view_name_id)
);
CREATE TABLE tarpitted_login_view(
    tarpit_release_date_time NUMERIC(20,0),

    view_name_id BIGINT NOT NULL,
    user_login_id BIGINT NOT NULL,
    PRIMARY KEY (view_name_id, user_login_id)
);



ALTER TABLE user_login_password_history ADD CONSTRAINT USER_LPH_USER
    FOREIGN KEY (user_login_id) REFERENCES user_login(user_login_id);

ALTER TABLE user_login_history ADD CONSTRAINT USER_LH_USER
    FOREIGN KEY (user_login_id) REFERENCES user_login(user_login_id);

ALTER TABLE user_login_session ADD CONSTRAINT USER_SESSION_USER
    FOREIGN KEY (user_login_id) REFERENCES user_login(user_login_id);


ALTER TABLE security_group_permission ADD CONSTRAINT SEC_GRP_PERM_GRP
    FOREIGN KEY (group_id) REFERENCES security_group(group_id);
ALTER TABLE security_group_permission ADD CONSTRAINT fk_213557788935327744
    FOREIGN KEY (permission_id) REFERENCES security_permission(permission_id);


ALTER TABLE user_login_security_group ADD CONSTRAINT USER_SECGRP_USER
    FOREIGN KEY (user_login_id) REFERENCES user_login(user_login_id);
ALTER TABLE user_login_security_group ADD CONSTRAINT USER_SECGRP_GRP
    FOREIGN KEY (group_id) REFERENCES security_group(group_id);

ALTER TABLE protected_view ADD CONSTRAINT VIEW_SECGRP_GRP
    FOREIGN KEY (group_id) REFERENCES security_group(group_id);

