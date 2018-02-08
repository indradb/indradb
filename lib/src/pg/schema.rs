pub const SCHEMA: &'static str = "
/* Vertices */
CREATE TABLE vertices (
    id UUID NOT NULL,
    type VARCHAR(1000) NOT NULL
);

ALTER TABLE vertices
    ADD CONSTRAINT vertices_pkey PRIMARY KEY (id);

/* Edges */
CREATE TABLE edges (
    id UUID NOT NULL,
    outbound_id UUID NOT NULL,
    type VARCHAR(1000) NOT NULL,
    inbound_id UUID NOT NULL,
    update_timestamp TIMESTAMP WITH TIME ZONE NOT NULL
);

ALTER TABLE edges
    ADD CONSTRAINT edges_pkey PRIMARY KEY (id),
    ADD CONSTRAINT edges_outbound_id_type_inbound_id_ukey UNIQUE (outbound_id, type, inbound_id),
    ADD CONSTRAINT edges_outbound_id_fkey FOREIGN KEY (outbound_id) REFERENCES vertices (id) ON DELETE CASCADE,
    ADD CONSTRAINT edges_inbound_id_fkey FOREIGN KEY (inbound_id) REFERENCES vertices (id) ON DELETE CASCADE;

CREATE INDEX ix_edges_update_timestamp ON edges USING btree (update_timestamp);
CREATE INDEX ix_edges_inbound_id ON edges USING btree (inbound_id);

/* Global metadata */
CREATE TABLE global_metadata (
    name VARCHAR(1024) NOT NULL,
    value JSONB NOT NULL
);

ALTER TABLE global_metadata
    ADD CONSTRAINT global_metadata_pkey PRIMARY KEY (name);

/* Vertex metadata */
CREATE TABLE vertex_metadata (
    owner_id UUID NOT NULL,
    name VARCHAR(1024) NOT NULL,
    value JSONB NOT NULL
);

ALTER TABLE vertex_metadata
    ADD CONSTRAINT vertex_metadata_pkey PRIMARY KEY (owner_id, name),
    ADD CONSTRAINT vertex_metadata_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES vertices (id) ON DELETE CASCADE;

/* Metadata */
CREATE TABLE edge_metadata (
    owner_id UUID NOT NULL,
    name VARCHAR(1024) NOT NULL,
    value JSONB NOT NULL
);

ALTER TABLE edge_metadata
    ADD CONSTRAINT edge_metadata_pkey PRIMARY KEY (owner_id, name),
    ADD CONSTRAINT edge_metadata_owner_id_fkey FOREIGN KEY (owner_id) REFERENCES edges (id) ON DELETE CASCADE;

";
