/* Accounts */
CREATE SEQUENCE account_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;

CREATE TABLE accounts (
    id INT DEFAULT nextval('account_id_seq'::regclass) NOT NULL,
    email VARCHAR(250) NOT NULL,
    salt VARCHAR(150) NOT NULL,
    api_secret_hash VARCHAR(150) NOT NULL
);

ALTER TABLE accounts
    ADD CONSTRAINT accounts_pkey PRIMARY KEY (id),
    ADD CONSTRAINT accounts_email_ukey UNIQUE (email);

/* Vertices */
CREATE SEQUENCE vertex_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1000;

CREATE TABLE vertices (
    id BIGINT DEFAULT nextval('vertex_id_seq'::regclass) NOT NULL,
    owner_id INT NOT NULL,
    type VARCHAR(1000) NOT NULL
);

ALTER TABLE vertices
    ADD CONSTRAINT vertices_pkey PRIMARY KEY (id),
    ADD CONSTRAINT vertices_owner_fkey FOREIGN KEY (owner_id) REFERENCES accounts (id) ON DELETE CASCADE;

/* Edges */
CREATE SEQUENCE edge_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1000;

CREATE TABLE edges (
    id BIGINT DEFAULT nextval('edge_id_seq'::regclass) NOT NULL,
    outbound_id BIGINT NOT NULL,
    type VARCHAR(1000) NOT NULL,
    inbound_id BIGINT NOT NULL,
    update_date TIMESTAMP NOT NULL,
    weight REAL NOT NULL
);

ALTER TABLE edges
    ADD CONSTRAINT edges_pkey PRIMARY KEY (id),
    ADD CONSTRAINT edges_outbound_id_type_inbound_id_ukey UNIQUE (outbound_id, type, inbound_id),
    ADD CONSTRAINT edges_outbound_id_fkey FOREIGN KEY (outbound_id) REFERENCES vertices (id) ON DELETE CASCADE,
    ADD CONSTRAINT edges_inbound_id_fkey FOREIGN KEY (inbound_id) REFERENCES vertices (id) ON DELETE CASCADE;

CREATE INDEX ix_edges_update_date ON edges USING btree (update_date);
CREATE INDEX ix_edges_inbound_id ON edges USING btree (inbound_id);

/* Global metadata */
CREATE SEQUENCE global_metadata_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;

CREATE TABLE global_metadata (
    id INT DEFAULT nextval('global_metadata_seq'::regclass) NOT NULL,
    key VARCHAR(1024) NOT NULL,
    value VARCHAR NOT NULL
);

ALTER TABLE global_metadata
    ADD CONSTRAINT global_metadata_pkey PRIMARY KEY (id),
    ADD CONSTRAINT global_metadata_key_ukey UNIQUE (key);

/* Account metadata */
CREATE SEQUENCE account_metadata_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;

CREATE TABLE account_metadata (
    id INT DEFAULT nextval('account_metadata_seq'::regclass) NOT NULL,
    owner_id INT NOT NULL,
    key VARCHAR(1024) NOT NULL,
    value VARCHAR NOT NULL
);

ALTER TABLE account_metadata
    ADD CONSTRAINT account_metadata_pkey PRIMARY KEY (id),
    ADD CONSTRAINT account_metadata_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES accounts (id) ON DELETE CASCADE,
    ADD CONSTRAINT account_metadata_owner_id_key_ukey UNIQUE (owner_id, key);

/* Vertex metadata */
CREATE SEQUENCE vertex_metadata_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;

CREATE TABLE vertex_metadata (
    id INT DEFAULT nextval('vertex_metadata_seq'::regclass) NOT NULL,
    owner_id BIGINT NOT NULL,
    key VARCHAR(1024) NOT NULL,
    value VARCHAR NOT NULL
);

ALTER TABLE vertex_metadata
    ADD CONSTRAINT vertex_metadata_pkey PRIMARY KEY (id),
    ADD CONSTRAINT vertex_metadata_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES vertices (id) ON DELETE CASCADE,
    ADD CONSTRAINT vertex_metadata_owner_id_key_ukey UNIQUE (owner_id, key);

/* Metadata */
CREATE SEQUENCE edge_metadata_seq
    START WITH 1
    INCREMENT BY 1
    NO MAXVALUE
    NO MINVALUE
    CACHE 1;

CREATE TABLE edge_metadata (
    id INT DEFAULT nextval('edge_metadata_seq'::regclass) NOT NULL,
    owner_id BIGINT NOT NULL,
    key VARCHAR(1024) NOT NULL,
    value VARCHAR NOT NULL
);

ALTER TABLE edge_metadata
    ADD CONSTRAINT edge_metadata_pkey PRIMARY KEY (id),
    ADD CONSTRAINT edge_metadata_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES edges (id) ON DELETE CASCADE,
    ADD CONSTRAINT edge_metadata_owner_id_key_ukey UNIQUE (owner_id, key);
