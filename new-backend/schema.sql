-- noinspection SqlNoDataSourceInspectionForFile

CREATE TABLE users
(
    id                         BIGINT PRIMARY KEY,
    pid                        UUID                     NOT NULL,
    email                      TEXT                     NOT NULL UNIQUE,
    name                       TEXT                     NOT NULL,
    password                   TEXT                     NOT NULL,
    reset_token                TEXT,
    reset_sent_at              timestamp with time zone,
    email_verification_token   TEXT,
    email_verification_sent_at timestamp with time zone,
    email_verified_at          timestamp with time zone,
    created_at                 timestamp with time zone NOT NULL,
    updated_at                 timestamp with time zone NOT NULL
);

CREATE TABLE sessions
(
    id          BIGINT PRIMARY KEY,
    user_id     BIGINT REFERENCES "users" (id) ON DELETE CASCADE NOT NULL,
    api_key     TEXT                                             NOT NULL UNIQUE,
    name        TEXT,
    description TEXT,
    platofrm    TEXT,
    created_at  timestamp with time zone                         NOT NULL,
    updated_at  timestamp with time zone                         NOT NULL
);

CREATE TABLE currencies
(
    id             BIGINT PRIMARY KEY,
    name           TEXT                     NOT NULL,
    symbol         TEXT                     NOT NULL,
    iso_code       TEXT,
    decimal_places INTEGER                  NOT NULL,
    user_id        BIGINT REFERENCES "users" (id) ON DELETE CASCADE,
    created_at     timestamp with time zone NOT NULL,
    updated_at     timestamp with time zone NOT NULL
);

CREATE TABLE currency_user_permissions
(
    user_id     BIGINT REFERENCES "users" (id) ON DELETE CASCADE    NOT NULL,
    currency_id BIGINT REFERENCES currencies (id) ON DELETE CASCADE NOT NULL,
    permissions INTEGER                                             NOT NULL,
    created_at  timestamp with time zone                            NOT NULL,
    updated_at  timestamp with time zone                            NOT NULL,
    PRIMARY KEY (user_id, currency_id)
);

CREATE TABLE linked_back_accounts
(
    id         BIGINT PRIMARY KEY,
    provider   TEXT                     NOT NULL,
    api_key    TEXT                     NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE TABLE linked_back_account_user_permissions
(
    user_id                BIGINT REFERENCES "users" (id) ON DELETE CASCADE              NOT NULL,
    linked_back_account_id BIGINT REFERENCES linked_back_accounts (id) ON DELETE CASCADE NOT NULL,
    permissions            INTEGER                                                       NOT NULL,
    created_at             timestamp with time zone                                      NOT NULL,
    updated_at             timestamp with time zone                                      NOT NULL,
    PRIMARY KEY (user_id, linked_back_account_id)
);

CREATE TABLE bank_accounts
(
    id                     BIGINT PRIMARY KEY,
    name                   TEXT                            NOT NULL,
    description            TEXT,
    iban                   TEXT UNIQUE,
    balance                BIGINT                          NOT NULL DEFAULT 0,
    original_balance       BIGINT                          NOT NULL DEFAULT 0,
    currency_id            BIGINT REFERENCES Currency (id) NOT NULL,
    linked_back_account_id BIGINT                          REFERENCES linked_back_accounts (id) ON DELETE SET NULL,
    created_at             timestamp with time zone        NOT NULL,
    updated_at             timestamp with time zone        NOT NULL,
);

CREATE TABLE bank_account_user_permissions
(
    user_id         BIGINT REFERENCES "users" (id) ON DELETE CASCADE       NOT NULL,
    bank_account_id BIGINT REFERENCES bank_accounts (id) ON DELETE CASCADE NOT NULL,
    permissions     INTEGER                                                NOT NULL,
    created_at      timestamp with time zone                               NOT NULL,
    updated_at      timestamp with time zone                               NOT NULL,
    PRIMARY KEY (user_id, bank_account_id)
);
