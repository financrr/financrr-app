-- ############################################################
-- #                                                          #
-- #                   Basic Entities                         #
-- #                                                          #
-- ############################################################

CREATE TABLE opensearch_migrations
(
    version     TEXT PRIMARY KEY UNIQUE,
    executed_at timestamp with time zone NOT NULL
);

CREATE TABLE instances
(
    node_id        SMALLINT PRIMARY KEY,
    last_heartbeat timestamp with time zone NOT NULL,
    created_at     timestamp with time zone NOT NULL,
    updated_at     timestamp with time zone NOT NULL
);

CREATE TABLE users
(
    id                         BIGINT PRIMARY KEY,
    email                      TEXT                     NOT NULL UNIQUE,
    name                       TEXT                     NOT NULL,
    flags                      INTEGER                  NOT NULL,
    password                   TEXT                     NOT NULL,
    reset_token                TEXT,
    reset_sent_at              timestamp with time zone,
    email_verification_token   TEXT,
    email_verification_sent_at timestamp with time zone,
    email_verified_at          timestamp with time zone,
    created_at                 timestamp with time zone NOT NULL,
    updated_at                 timestamp with time zone NOT NULL,
    UNIQUE (email, email_verification_token),
    UNIQUE (email, reset_token)
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

CREATE INDEX idx_user_permissions_user_id ON user_permissions (user_id);
CREATE INDEX idx_user_permissions_entity ON user_permissions (entity_type, entity_id);

CREATE TABLE sessions
(
    id               BIGINT PRIMARY KEY,
    user_id          BIGINT REFERENCES "users" (id) ON DELETE CASCADE NOT NULL,
    api_key          TEXT                                             NOT NULL UNIQUE,
    name             TEXT,
    user_agent       TEXT,
    last_accessed_at timestamp with time zone,
    created_at       timestamp with time zone                         NOT NULL,
    updated_at       timestamp with time zone                         NOT NULL
);

CREATE INDEX idx_sessions_user_id ON sessions (user_id);
CREATE INDEX idx_sessions_api_key ON sessions (api_key);

CREATE TABLE file_attachments
(
    id                  BIGINT PRIMARY KEY,
    globally_accessible BOOLEAN                  NOT NULL,
    name                TEXT                     NOT NULL,
    path                TEXT                     NOT NULL,
    type                TEXT                     NOT NULL,
    size                BIGINT                   NOT NULL,
    created_at          timestamp with time zone NOT NULL,
    updated_at          timestamp with time zone NOT NULL
);

CREATE INDEX idx_file_attachments_name ON file_attachments (name);

CREATE TABLE currencies
(
    id             BIGINT PRIMARY KEY,
    user_id        BIGINT REFERENCES "users" (id) ON DELETE CASCADE,
    name           TEXT                     NOT NULL,
    symbol         TEXT                     NOT NULL,
    iso_code       TEXT,
    decimal_places INTEGER                  NOT NULL,
    created_at     timestamp with time zone NOT NULL,
    updated_at     timestamp with time zone NOT NULL
);

CREATE TABLE categories
(
    id         BIGINT PRIMARY KEY,
    parent_id  BIGINT REFERENCES categories (id) ON DELETE CASCADE,
    user_id    BIGINT REFERENCES "users" (id) ON DELETE CASCADE,
    name       TEXT                     NOT NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE INDEX idx_categories_user_id ON categories (user_id);
CREATE INDEX idx_categories_parent_id ON categories (parent_id);

CREATE TABLE tags
(
    id         BIGINT PRIMARY KEY,
    user_id    BIGINT REFERENCES "users" (id) ON DELETE CASCADE NOT NULL,
    name       TEXT                                             NOT NULL,
    created_at timestamp with time zone                         NOT NULL,
    updated_at timestamp with time zone                         NOT NULL
);

CREATE INDEX idx_tags_user_id ON tags (user_id);

CREATE TABLE taggings
(
    tag_id      BIGINT REFERENCES tags (id) ON DELETE CASCADE NOT NULL,
    entity_type TEXT                                          NOT NULL,
    entity_id   BIGINT                                        NOT NULL,
    created_at  timestamp with time zone                      NOT NULL,
    updated_at  timestamp with time zone                      NOT NULL,
    PRIMARY KEY (tag_id, entity_type, entity_id)
);

CREATE INDEX idx_taggings_tag_id ON taggings (tag_id);
CREATE INDEX idx_taggings_entity ON taggings (entity_type, entity_id);

-- ############################################################
-- #                                                          #
-- #              External Bank Institutions                  #
-- #                                                          #
-- ############################################################

CREATE TABLE external_bank_institutions
(
    id                     BIGINT PRIMARY KEY,
    external_id            TEXT                     NOT NULL,
    provider               TEXT                     NOT NULL,
    name                   TEXT                     NOT NULL,
    bic                    TEXT,
    countries              TEXT[]                   NOT NULL,
    logo_link              TEXT,
    access_valid_for_days  INT,
    transaction_total_days INT,
    created_at             timestamp with time zone NOT NULL,
    updated_at             timestamp with time zone NOT NULL,
    UNIQUE (external_id, provider)
);

-- ############################################################
-- #                                                          #
-- #                      GoCardless                          #
-- #                                                          #
-- ############################################################

CREATE TABLE go_cardless_enduser_agreements
(
    id                           BIGINT PRIMARY KEY,
    external_id                  TEXT UNIQUE                                       NOT NULL,
    external_bank_institution_id BIGINT REFERENCES external_bank_institutions (id) NOT NULL UNIQUE,
    max_historical_days          INT                                               NOT NULL,
    access_valid_for_days        INT                                               NOT NULL,
    created_at                   timestamp with time zone                          NOT NULL,
    updated_at                   timestamp with time zone                          NOT NULL
);

CREATE TABLE go_cardless_requisitions
(
    id                           BIGINT PRIMARY KEY,
    external_id                  TEXT UNIQUE                                           NOT NULL,
    link                         TEXT UNIQUE                                           NOT NULL,
    agreement_id                 BIGINT REFERENCES go_cardless_enduser_agreements (id) NOT NULL,
    external_bank_institution_id BIGINT REFERENCES external_bank_institutions (id)     NOT NULL,
    user_id                      BIGINT REFERENCES users (id)                          NOT NULL,
    used_at                      timestamp with time zone,
    created_at                   timestamp with time zone                              NOT NULL,
    updated_at                   timestamp with time zone                              NOT NULL
);

-- ############################################################
-- #                                                          #
-- #                External Bank Accounts                    #
-- #                                                          #
-- ############################################################

CREATE TABLE external_bank_accounts
(
    id         BIGINT PRIMARY KEY,
    name       TEXT                     NOT NULL,
    logo_id    BIGINT                   REFERENCES file_attachments (id) ON DELETE SET NULL,
    created_at timestamp with time zone NOT NULL,
    updated_at timestamp with time zone NOT NULL
);

CREATE TABLE external_bank_account_ibans
(
    id                       BIGINT PRIMARY KEY,
    external_bank_account_id BIGINT REFERENCES external_bank_accounts (id) ON DELETE CASCADE NOT NULL,
    iban                     TEXT                                                            NOT NULL UNIQUE,
    created_at               timestamp with time zone                                        NOT NULL,
    updated_at               timestamp with time zone                                        NOT NULL
);

CREATE INDEX idx_external_bank_account_ibans_account_id ON external_bank_account_ibans (external_bank_account_id);
CREATE INDEX idx_external_bank_account_ibans_iban ON external_bank_account_ibans (iban);

-- ############################################################
-- #                                                          #
-- #                   Bank Accounts                          #
-- #                                                          #
-- ############################################################

CREATE TABLE linked_back_accounts
(
    id             BIGINT PRIMARY KEY,
    external_id    TEXT                     NOT NULL,
    provider       TEXT                     NOT NULL,
    effective_iban TEXT UNIQUE              NOT NULL,
    created_at     timestamp with time zone NOT NULL,
    updated_at     timestamp with time zone NOT NULL,
    UNIQUE (provider, external_id)
);

CREATE TABLE bank_accounts
(
    id                     BIGINT PRIMARY KEY,
    currency_id            BIGINT REFERENCES currencies (id) UNIQUE NOT NULL,
    linked_back_account_id BIGINT                                   REFERENCES linked_back_accounts (id) ON DELETE SET NULL,
    name                   TEXT                                     NOT NULL,
    description            TEXT,
    iban                   TEXT,
    balance                BIGINT                                   NOT NULL DEFAULT 0,
    original_balance       BIGINT                                   NOT NULL DEFAULT 0,
    created_at             timestamp with time zone                 NOT NULL,
    updated_at             timestamp with time zone                 NOT NULL
);

CREATE INDEX idx_bank_accounts_iban ON bank_accounts (iban);

-- ############################################################
-- #                                                          #
-- #                     Transactions                         #
-- #                                                          #
-- ############################################################

CREATE TYPE transaction_type AS ENUM ('income', 'expense', 'transfer');

CREATE TABLE transaction_parties
(
    id                       BIGINT PRIMARY KEY,
    bank_account_id          BIGINT REFERENCES bank_accounts (id),
    external_bank_account_id BIGINT REFERENCES external_bank_accounts (id),
    created_at               timestamp with time zone NOT NULL,
    updated_at               timestamp with time zone NOT NULL
);

CREATE TABLE transactions
(
    id                 BIGINT PRIMARY KEY,
    source_id          BIGINT REFERENCES transaction_parties (id),
    destination_id     BIGINT REFERENCES transaction_parties (id),
    currency_id        BIGINT REFERENCES currencies (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    category_id        BIGINT                                                                REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL,
    file_attachment_id BIGINT                                                                REFERENCES file_attachments (id) ON DELETE SET NULL,
    source_name        TEXT,
    source_iban        TEXT,
    destination_name   TEXT,
    destination_iban   TEXT,
    type               transaction_type                                                      NOT NULL,
    amount             BIGINT                                                                NOT NULL,
    name               TEXT                                                                  NOT NULL,
    purpose            TEXT,
    note               TEXT,
    booking_date       timestamp with time zone,
    created_at         timestamp with time zone                                              NOT NULL,
    updated_at         timestamp with time zone                                              NOT NULL
);

CREATE INDEX idx_transactions_source_id ON transactions (source_id);
CREATE INDEX idx_transactions_destination_id ON transactions (destination_id);
CREATE INDEX idx_transactions_category_id ON transactions (category_id);

CREATE TABLE transaction_templates
(
    id                 BIGINT PRIMARY KEY,
    source_id          BIGINT REFERENCES transaction_parties (id),
    destination_id     BIGINT REFERENCES transaction_parties (id),
    currency_id        BIGINT REFERENCES currencies (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    category_id        BIGINT                                                                REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL,
    file_attachment_id BIGINT                                                                REFERENCES file_attachments (id) ON DELETE SET NULL,
    source_name        TEXT,
    source_iban        TEXT,
    destination_name   TEXT,
    destination_iban   TEXT,
    type               transaction_type                                                      NOT NULL,
    amount             BIGINT                                                                NOT NULL,
    name               TEXT                                                                  NOT NULL,
    purpose            TEXT,
    note               TEXT,
    created_at         timestamp with time zone                                              NOT NULL,
    updated_at         timestamp with time zone                                              NOT NULL
);

CREATE INDEX idx_transaction_templates_source_id ON transaction_templates (source_id);
CREATE INDEX idx_transaction_templates_destination_id ON transaction_templates (destination_id);
CREATE INDEX idx_transaction_templates_category_id ON transaction_templates (category_id);

CREATE TABLE recurring_transactions
(
    id                  BIGINT PRIMARY KEY,
    source_id           BIGINT REFERENCES transaction_parties (id),
    destination_id      BIGINT REFERENCES transaction_parties (id),
    currency_id         BIGINT REFERENCES currencies (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    category_id         BIGINT                                                                REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL,
    file_attachment_id  BIGINT                                                                REFERENCES file_attachments (id) ON DELETE SET NULL,
    source_name         TEXT,
    source_iban         TEXT,
    destination_name    TEXT,
    destination_iban    TEXT,
    type                transaction_type                                                      NOT NULL,
    amount              BIGINT                                                                NOT NULL,
    name                TEXT                                                                  NOT NULL,
    purpose             TEXT,
    note                TEXT,
    cron                TEXT                                                                  NOT NULL,
    executions_per_year REAL                                                                  NOT NULL,
    last_executed_at    timestamp with time zone,
    created_at          timestamp with time zone                                              NOT NULL,
    updated_at          timestamp with time zone                                              NOT NULL
);

CREATE INDEX idx_recurring_transactions_source_id ON recurring_transactions (source_id);
CREATE INDEX idx_recurring_transactions_destination_id ON recurring_transactions (destination_id);
CREATE INDEX idx_recurring_transactions_category_id ON recurring_transactions (category_id);

CREATE TABLE pending_transactions
(
    id                 BIGINT PRIMARY KEY,
    source_id          BIGINT REFERENCES transaction_parties (id),
    destination_id     BIGINT REFERENCES transaction_parties (id),
    currency_id        BIGINT REFERENCES currencies (id) ON UPDATE CASCADE ON DELETE CASCADE NOT NULL,
    category_id        BIGINT                                                                REFERENCES categories (id) ON UPDATE SET NULL ON DELETE SET NULL,
    file_attachment_id BIGINT                                                                REFERENCES file_attachments (id) ON DELETE SET NULL,
    source_name        TEXT,
    source_iban        TEXT,
    destination_name   TEXT,
    destination_iban   TEXT,
    type               transaction_type                                                      NOT NULL,
    amount             BIGINT                                                                NOT NULL,
    name               TEXT                                                                  NOT NULL,
    purpose            TEXT,
    note               TEXT,
    value_date         timestamp with time zone,
    created_at         timestamp with time zone                                              NOT NULL,
    updated_at         timestamp with time zone                                              NOT NULL
);

CREATE INDEX idx_pending_transactions_source_id ON pending_transactions (source_id);
CREATE INDEX idx_pending_transactions_destination_id ON pending_transactions (destination_id);
CREATE INDEX idx_pending_transactions_category_id ON pending_transactions (category_id);

-- ############################################################
-- #                                                          #
-- #                      Contracts                           #
-- #                                                          #
-- ############################################################

CREATE TABLE contracts
(
    id                       BIGINT PRIMARY KEY,
    recurring_transaction_id BIGINT REFERENCES recurring_transactions (id) ON DELETE CASCADE NOT NULL,
    category_id              BIGINT                                                          REFERENCES categories (id) ON DELETE SET NULL,
    name                     TEXT                                                            NOT NULL,
    description              TEXT,
    created_at               timestamp with time zone                                        NOT NULL,
    updated_at               timestamp with time zone                                        NOT NULL
);

CREATE INDEX idx_contracts_recurring_transaction_id ON contracts (recurring_transaction_id);
CREATE INDEX idx_contracts_category_id ON contracts (category_id);

CREATE TABLE inactive_contracts
(
    id                  BIGINT PRIMARY KEY,
    last_transaction_id BIGINT REFERENCES transactions (id) ON DELETE CASCADE NOT NULL,
    category_id         BIGINT                                                REFERENCES categories (id) ON DELETE SET NULL,
    canceled_at         timestamp with time zone                              NOT NULL,
    name                TEXT                                                  NOT NULL,
    description         TEXT,
    created_at          timestamp with time zone                              NOT NULL,
    updated_at          timestamp with time zone                              NOT NULL
);

CREATE INDEX idx_inactive_contracts_last_transaction_id ON inactive_contracts (last_transaction_id);
CREATE INDEX idx_inactive_contracts_category_id ON inactive_contracts (category_id);


-- ############################################################
-- #                                                          #
-- #                         Criteria                         #
-- #                                                          #
-- ############################################################

CREATE TYPE filter_transaction_type AS ENUM ('all', 'contracts', 'non-contracts');

CREATE TABLE budget_criteria
(
    id                         BIGINT PRIMARY KEY,
    all_categories             BOOLEAN                  NOT NULL,
    all_tags                   BOOLEAN                  NOT NULL,
    all_external_bank_accounts BOOLEAN                  NOT NULL,
    all_bank_accounts          BOOLEAN                  NOT NULL,
    transaction_type           filter_transaction_type  NOT NULL,
    created_at                 timestamp with time zone NOT NULL,
    updated_at                 timestamp with time zone NOT NULL
);

CREATE TABLE budget_criteria_categories
(
    budget_criteria_id BIGINT REFERENCES budget_criteria (id) ON DELETE CASCADE,
    category_id        BIGINT REFERENCES categories (id) ON DELETE CASCADE,
    created_at         timestamp with time zone NOT NULL,
    updated_at         timestamp with time zone NOT NULL,
    PRIMARY KEY (budget_criteria_id, category_id)
);

CREATE INDEX idx_budget_criteria_categories_budget_criteria_id ON budget_criteria_categories (budget_criteria_id);
CREATE INDEX idx_budget_criteria_categories_category_id ON budget_criteria_categories (category_id);

CREATE TABLE budget_criteria_tags
(
    budget_criteria_id BIGINT REFERENCES budget_criteria (id) ON DELETE CASCADE,
    tag_id             BIGINT REFERENCES tags (id) ON DELETE CASCADE,
    created_at         timestamp with time zone NOT NULL,
    updated_at         timestamp with time zone NOT NULL,
    PRIMARY KEY (budget_criteria_id, tag_id)
);

CREATE INDEX idx_budget_criteria_tags_budget_criteria_id ON budget_criteria_tags (budget_criteria_id);
CREATE INDEX idx_budget_criteria_tags_tag_id ON budget_criteria_tags (tag_id);

CREATE TABLE budget_criteria_external_bank_accounts
(
    budget_criteria_id       BIGINT REFERENCES budget_criteria (id) ON DELETE CASCADE,
    external_bank_account_id BIGINT REFERENCES external_bank_accounts (id) ON DELETE CASCADE,
    created_at               timestamp with time zone NOT NULL,
    updated_at               timestamp with time zone NOT NULL,
    PRIMARY KEY (budget_criteria_id, external_bank_account_id)
);

CREATE INDEX idx_budget_criteria_external_bank_accounts_budget_criteria_id ON budget_criteria_external_bank_accounts (budget_criteria_id);
CREATE INDEX idx_budget_criteria_external_bank_accounts_external_bank_account_id ON budget_criteria_external_bank_accounts (external_bank_account_id);

CREATE TABLE budget_criteria_bank_accounts
(
    budget_criteria_id BIGINT REFERENCES budget_criteria (id) ON DELETE CASCADE,
    bank_account_id    BIGINT REFERENCES bank_accounts (id) ON DELETE CASCADE,
    created_at         timestamp with time zone NOT NULL,
    updated_at         timestamp with time zone NOT NULL,
    PRIMARY KEY (budget_criteria_id, bank_account_id)
);

CREATE INDEX idx_budget_criteria_bank_accounts_budget_criteria_id ON budget_criteria_bank_accounts (budget_criteria_id);
CREATE INDEX idx_budget_criteria_bank_accounts_bank_account_id ON budget_criteria_bank_accounts (bank_account_id);

-- ############################################################
-- #                                                          #
-- #                          Budget                          #
-- #                                                          #
-- ############################################################

CREATE TYPE budget_type AS ENUM ('resetting', 'accumulating');

CREATE TABLE budgets
(
    id             BIGINT PRIMARY KEY,
    criteria_id    BIGINT REFERENCES budget_criteria (id) NOT NULL,
    type           budget_type                            NOT NULL,
    current_amount BIGINT                                 NOT NULL,
    amount         BIGINT                                 NOT NULL,
    cron           TEXT                                   NOT NULL,
    name           TEXT                                   NOT NULL,
    description    TEXT,
    map_all        BOOLEAN                                NOT NULL DEFAULT FALSE,
    created_at     timestamp with time zone               NOT NULL,
    updated_at     timestamp with time zone               NOT NULL
);

CREATE TABLE budget_histories
(
    id             BIGINT PRIMARY KEY,
    budget_id      BIGINT                   NOT NULL,
    budget_type    budget_type              NOT NULL,
    current_amount BIGINT                   NOT NULL,
    amount         BIGINT                   NOT NULL,
    cron           TEXT                     NOT NULL,
    name           TEXT                     NOT NULL,
    description    TEXT,
    map_all        BOOLEAN                  NOT NULL DEFAULT FALSE,
    recorded_at    timestamp with time zone NOT NULL,
    created_at     timestamp with time zone NOT NULL,
    updated_at     timestamp with time zone NOT NULL
);

CREATE INDEX idx_budget_history_budget_id ON budget_histories (budget_id);
