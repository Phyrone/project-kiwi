CREATE EXTENSION citext;
CREATE DOMAIN email AS citext
    CHECK ( value ~
            '^[a-zA-Z0-9.!#$%&''*+/=?^_`{|}~-]+@[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?(?:\.[a-zA-Z0-9](?:[a-zA-Z0-9-]{0,61}[a-zA-Z0-9])?)*$' );


CREATE TABLE "node"
(
    "id"         BIGSERIAL PRIMARY KEY,
    "first_seen" TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "domain"     VARCHAR(255) NOT NULL
);

CREATE TABLE "user"
(
    "id"             BIGSERIAL PRIMARY KEY,
    "email"          EMAIL UNIQUE NOT NULL,
    "email_verified" BOOLEAN               DEFAULT FALSE,
    "password"       TEXT                  DEFAULT NULL,
    "session_secret" BYTEA        NULL,
    "created_at"     TIMESTAMPTZ  NOT NULL DEFAULT CURRENT_TIMESTAMP
);

CREATE TABLE "user_key"
(
    "id"          BIGSERIAL PRIMARY KEY,
    "user"        BIGINT      NOT NULL REFERENCES "user" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    "created_at"  TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "credentials" JSONB       NOT NULL
);

CREATE TABLE "profile"
(
    "id"            BIGSERIAL PRIMARY KEY,
    "name"          TEXT NOT NULL,
    "discriminator" INT2  DEFAULT NULL,
    "displayname"   TEXT NULL,
    "owning_user"   BIGINT REFERENCES "user" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    "owning_node"   BIGINT REFERENCES "node" (id) ON DELETE CASCADE ON UPDATE CASCADE,

    "metadata"      JSONB DEFAULT '{}'::JSONB,
    UNIQUE NULLS NOT DISTINCT (name, discriminator),
    CHECK ( (owning_node IS NOT NULL)::int + (owning_user IS NOT NULL)::int = 1 )
);



CREATE TABLE "guild"
(
    "id"         BIGSERIAL PRIMARY KEY,
    "name"       TEXT        NOT NULL,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "owner"      BIGINT REFERENCES "profile" (id)
);

CREATE TABLE "channel"
(
    "id"         BIGSERIAL PRIMARY KEY,
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,

    "guild"      BIGINT      NOT NULL REFERENCES "guild" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    "parent"     BIGINT      NULL REFERENCES "channel" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    "name"       TEXT        NOT NULL,
    "order"      INT4        NOT NULL DEFAULT 0

);

-- All types of publications from chat messages, over comments to posts

CREATE TABLE "post"
(
    "id"         BIGSERIAL PRIMARY KEY,
    "author"     BIGINT      NOT NULL REFERENCES "profile" (id),
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "title"      TEXT        NOT NULL,
    "body"       JSONB                DEFAULT NULL
);

CREATE TABLE "post_attachment"
(
    "id"          BIGSERIAL PRIMARY KEY,
    "publication" BIGINT NOT NULL REFERENCES "post" (id) ON DELETE CASCADE ON UPDATE CASCADE
);

CREATE TABLE "comment"
(
    "id"         BIGSERIAL PRIMARY KEY,
    "author"     BIGINT      NOT NULL REFERENCES "profile" (id),
    "created_at" TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
    "post"       BIGINT      NOT NULL REFERENCES "post" (id),
    "reply_to"   BIGINT      NULL REFERENCES "comment" (id) ON DELETE CASCADE ON UPDATE CASCADE,
    "body"       TEXT        NOT NULL
);