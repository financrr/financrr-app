-- ############################################################
-- #                                                          #
-- #                   Basic Entities                         #
-- #                                                          #
-- ############################################################

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

CREATE TABLE user_permissions
(
    user_id     BIGINT REFERENCES "users" (id) ON DELETE CASCADE NOT NULL,
    entity_type TEXT                                             NOT NULL,
    entity_id   BIGINT                                           NOT NULL,
    permissions INTEGER                                          NOT NULL,
    created_at  timestamp with time zone                         NOT NULL,
    updated_at  timestamp with time zone                         NOT NULL,
    PRIMARY KEY (user_id, entity_type, entity_id)
);

CREATE TABLE sessions
(
    id          BIGINT PRIMARY KEY,
    user_id     BIGINT REFERENCES "users" (id) ON DELETE CASCADE NOT NULL,
    api_key     TEXT                                             NOT NULL UNIQUE,
    name        TEXT,
    description TEXT,
    platform    TEXT,
    created_at  timestamp with time zone                         NOT NULL,
    updated_at  timestamp with time zone                         NOT NULL
);

CREATE TABLE file_attachments
(
    id         BIGINT PRIMARY KEY,
    name       TEXT                     NOT NULL,
    path       TEXT                     NOT NULL,
    type       TEXT                     NOT NULL,
    size       BIGINT                   NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
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

CREATE TABLE categories
(
    id         BIGINT PRIMARY KEY,
    name       TEXT                     NOT NULL,
    parent_id  BIGINT REFERENCES categories (id) ON DELETE CASCADE,
    user_id    BIGINT REFERENCES "users" (id) ON DELETE CASCADE,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE TABLE tags
(
    id         BIGINT PRIMARY KEY,
    name       TEXT                                             NOT NULL,
    user_id    BIGINT REFERENCES "users" (id) ON DELETE CASCADE NOT NULL,
    created_at timestamp with time zone                         NOT NULL,
    updated_at timestamp with time zone                         NOT NULL
);

CREATE TABLE taggings
(
    tag_id      BIGINT REFERENCES tags (id) ON DELETE CASCADE NOT NULL,
    entity_type TEXT                                          NOT NULL,
    entity_id   BIGINT                                        NOT NULL,
    created_at  timestamp with time zone                      NOT NULL,
    updated_at  timestamp with time zone                      NOT NULL,
    PRIMARY KEY (tag_id, entity_type, entity_id)
);

CREATE TABLE linked_back_accounts
(
    id         BIGINT PRIMARY KEY,
    provider   TEXT                     NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE TABLE bank_accounts
(
    id                     BIGINT PRIMARY KEY,
    name                   TEXT                              NOT NULL,
    description            TEXT,
    iban                   TEXT UNIQUE,
    balance                BIGINT                            NOT NULL DEFAULT 0,
    original_balance       BIGINT                            NOT NULL DEFAULT 0,
    currency_id            BIGINT REFERENCES currencies (id) NOT NULL,
    linked_back_account_id BIGINT                            REFERENCES linked_back_accounts (id) ON DELETE SET NULL,
    global_accessible      BOOLEAN                           NOT NULL,
    created_at             timestamp with time zone          NOT NULL,
    updated_at             timestamp with time zone          NOT NULL
);

-- ############################################################
-- #                                                          #
-- #                     Transactions                         #
-- #                                                          #
-- ############################################################

CREATE TABLE base_transactions
(
    id                 BIGINT PRIMARY KEY,
    type               TEXT                                                                  NOT NULL,
    source             BIGINT REFERENCES bank_accounts (id) ON UPDATE CASCADE ON DELETE CASCADE,
    destination        BIGINT REFERENCES bank_accounts (id) ON UPDATE CASCADE ON DELETE CASCADE,
    amount             BIGINT                                                                NOT NULL,
    currency           BIGINT REFERENCES currencies (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    category_id        BIGINT                                                                REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL,
    name               TEXT                                                                  NOT NULL,
    purpose            TEXT,
    note               TEXT,
    file_attachment_id BIGINT                                                                REFERENCES file_attachments (id) ON DELETE SET NULL,
    created_at         timestamp with time zone                                              NOT NULL,
    updated_at         timestamp with time zone                                              NOT NULL
);

CREATE TABLE transactions
(
    type        TEXT,
    executed_at timestamp with time zone,
    PRIMARY KEY (id)
) INHERITS (base_transactions);

CREATE TABLE transaction_templates
(
    PRIMARY KEY (id)
) INHERITS (base_transactions);

CREATE TABLE recurring_transactions
(
    cron             TEXT NOT NULL,
    last_executed_at timestamp with time zone,
    PRIMARY KEY (id)
) INHERITS (base_transactions);

CREATE TABLE scheduled_transactions
(
    scheduled_at timestamp with time zone,
    PRIMARY KEY (id)
) INHERITS (base_transactions);

-- ############################################################
-- #                                                          #
-- #                      Contracts                           #
-- #                                                          #
-- ############################################################

CREATE TABLE base_contracts
(
    id          BIGINT PRIMARY KEY,
    name        TEXT                     NOT NULL,
    description TEXT,
    category_id BIGINT                   REFERENCES categories (id) ON DELETE SET NULL,
    created_at  timestamp with time zone NOT NULL,
    updated_at  timestamp with time zone NOT NULL
);

CREATE TABLE contracts
(
    recurring_transaction_id BIGINT REFERENCES recurring_transactions (id) ON DELETE CASCADE NOT NULL,
    PRIMARY KEY (id)
) INHERITS (base_contracts);

CREATE TABLE inactive_contracts
(
    canceled_at         timestamp with time zone                              NOT NULL,
    last_transaction_id BIGINT REFERENCES transactions (id) ON DELETE CASCADE NOT NULL,
    PRIMARY KEY (id)
) INHERITS (base_contracts);

-- ############################################################
-- #                                                          #
-- #                          Budget                          #
-- #                                                          #
-- ############################################################

CREATE TABLE base_budgets
(
    id             BIGINT PRIMARY KEY,
    monthly_amount BIGINT                   NOT NULL,
    correction     BIGINT                   NOT NULL DEFAULT 0,
    name           TEXT                     NOT NULL,
    description    TEXT,
    created_at     timestamp with time zone NOT NULL,
    updated_at     timestamp with time zone NOT NULL
);

CREATE TABLE monthly_budgets
(
    PRIMARY KEY (id)
) INHERITS (base_budgets);

CREATE TABLE savings_budgets
(
    total_amount BIGINT NOT NULL,
    PRIMARY KEY (id)
) INHERITS (base_budgets);