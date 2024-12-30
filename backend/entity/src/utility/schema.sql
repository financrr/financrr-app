-- =============================================
-- =               DOMAINS                     =
-- =============================================

DO
$$
    BEGIN
        CREATE DOMAIN uint32 AS bigint
            CHECK (VALUE >= 0 AND VALUE < 4294967296);
    EXCEPTION
        WHEN duplicate_object THEN null;
    END
$$;

-- =============================================
-- =               TABLES                      =
-- =============================================

CREATE TABLE IF NOT EXISTS "user"
(
    id           BIGINT PRIMARY KEY,
    username     TEXT UNIQUE              NOT NULL,
    email        TEXT UNIQUE,
    display_name TEXT,
    password     TEXT                     NOT NULL,
    created_at   timestamp with time zone NOT NULL DEFAULT current_timestamp,
    is_admin     BOOLEAN                  NOT NULL DEFAULT FALSE
);

CREATE TABLE IF NOT EXISTS session
(
    id          BIGINT PRIMARY KEY,
    token       TEXT UNIQUE                                                        NOT NULL,
    name        TEXT                                                               NOT NULL,
    description TEXT,
    platform    TEXT,
    "user"      BIGINT REFERENCES "user" (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    created_at  timestamp with time zone                                           NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS permissions
(
    user_id     BIGINT REFERENCES "user" (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    entity_type TEXT                                                              NOT NULL,
    entity_id   BIGINT                                                            NOT NULL,
    permissions INTEGER                                                           NOT NULL,
    PRIMARY KEY (user_id, entity_type, entity_id)
);

CREATE TABLE IF NOT EXISTS currency
(
    id             BIGINT PRIMARY KEY,
    name           TEXT    NOT NULL,
    symbol         TEXT    NOT NULL,
    iso_code       TEXT,
    decimal_places INTEGER NOT NULL,
    "user"         BIGINT REFERENCES "user" (id) ON UPDATE CASCADE ON DELETE CASCADE
);

CREATE TABLE IF NOT EXISTS account
(
    id               BIGINT PRIMARY KEY,
    name             TEXT                                                                NOT NULL,
    description      TEXT,
    iban             TEXT UNIQUE,
    balance          BIGINT                                                              NOT NULL DEFAULT 0,
    original_balance BIGINT                                                              NOT NULL DEFAULT 0,
    currency         BIGINT REFERENCES Currency (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    created_at       timestamp with time zone                                            NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS budget
(
    id          BIGINT PRIMARY KEY,
    "user"      BIGINT REFERENCES "user" (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    amount      BIGINT                                                            NOT NULL,
    name        TEXT                                                              NOT NULL,
    description TEXT,
    created_at  timestamp with time zone                                          NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS transaction_template
(
    id          BIGINT PRIMARY KEY,
    source      BIGINT REFERENCES account (id) ON UPDATE CASCADE ON DELETE CASCADE,
    destination BIGINT REFERENCES account (id) ON UPDATE CASCADE ON DELETE CASCADE,
    amount      BIGINT                                                              NOT NULL,
    currency    BIGINT REFERENCES currency (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    name        TEXT                                                                NOT NULL,
    description TEXT,
    budget      BIGINT REFERENCES budget (id) ON UPDATE CASCADE ON DELETE CASCADE,
    created_at  timestamp with time zone                                            NOT NULL DEFAULT current_timestamp,

    CHECK (source IS NOT NULL OR destination IS NOT NULL)
);

CREATE TABLE IF NOT EXISTS recurring_transaction
(
    id               BIGINT PRIMARY KEY,
    template         BIGINT REFERENCES transaction_template (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    recurring_rule   json                                                                            NOT NULL,
    last_executed_at timestamp with time zone,
    created_at       timestamp with time zone                                                        NOT NULL DEFAULT current_timestamp
);

CREATE TABLE IF NOT EXISTS transaction
(
    id          BIGINT PRIMARY KEY,
    source      BIGINT REFERENCES Account (id) ON UPDATE CASCADE ON DELETE CASCADE,
    destination BIGINT REFERENCES Account (id) ON UPDATE CASCADE ON DELETE CASCADE,
    amount      BIGINT                                                              NOT NULL,
    currency    BIGINT REFERENCES Currency (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    name        TEXT                                                                NOT NULL,
    description TEXT,
    budget      BIGINT REFERENCES budget (id) ON UPDATE CASCADE ON DELETE CASCADE,
    executed_at timestamp with time zone                                            NOT NULL DEFAULT current_timestamp,
    created_at  timestamp with time zone                                            NOT NULL DEFAULT current_timestamp,
    CHECK (source IS NOT NULL OR destination IS NOT NULL)
);
