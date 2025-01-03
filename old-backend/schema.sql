CREATE TABLE IF NOT EXISTS "user"
(
    id           BIGINT PRIMARY KEY,
    username     TEXT UNIQUE NOT NULL,
    display_name TEXT,
    email        TEXT UNIQUE,
    password     TEXT        NOT NULL,
    permissions  INTEGER     NOT NULL
);

CREATE TABLE IF NOT EXISTS session
(
    id          BIGINT PRIMARY KEY,
    token       TEXT UNIQUE                                                       NOT NULL,
    name        TEXT                                                              NOT NULL,
    description TEXT,
    platform    TEXT,
    "user"      BIGINT REFERENCES "user" (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL
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

CREATE TABLE IF NOT EXISTS currency_conversion_rate
(
    source_currency BIGINT REFERENCES currency (id) ON UPDATE CASCADE ON DELETE CASCADE,
    target_currency BIGINT REFERENCES currency (id) ON UPDATE CASCADE ON DELETE CASCADE,
    conversion_rate INTEGER NOT NULL,
    PRIMARY KEY (source_currency, target_currency)
);

CREATE TABLE IF NOT EXISTS account
(
    id               BIGINT PRIMARY KEY,
    name             TEXT                                                                NOT NULL,
    description      TEXT,
    iban             TEXT UNIQUE,
    balance          BIGINT                                                              NOT NULL DEFAULT 0,
    original_balance BIGINT                                                              NOT NULL DEFAULT 0,
    currency         BIGINT REFERENCES Currency (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL
);

CREATE TABLE IF NOT EXISTS budget
(
    id          BIGINT PRIMARY KEY,
    "user"      BIGINT REFERENCES "user" (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    amount      BIGINT                                                            NOT NULL,
    name        TEXT                                                              NOT NULL,
    description TEXT
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
    budget      BIGINT                                                              REFERENCES budget (id) ON UPDATE SET NULL ON DELETE SET NULL
);

CREATE TABLE IF NOT EXISTS recurring_transaction
(
    id               BIGINT PRIMARY KEY,
    template         BIGINT REFERENCES transaction_template (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    recurring_rule   json                                                                            NOT NULL,
    last_executed_at timestamp with time zone
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
    budget      BIGINT                                                              REFERENCES budget (id) ON UPDATE SET NULL ON DELETE SET NULL,
    executed_at timestamp with time zone                                            NOT NULL DEFAULT current_timestamp
);
