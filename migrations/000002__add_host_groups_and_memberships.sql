-- 000002__add_host_groups_and_memberships.sql
-- NIST-compliant extension for dynamic host groups + many-to-many

CREATE TABLE host_groups (
    id              SERIAL PRIMARY KEY,
    organization_id INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    name            TEXT NOT NULL,           -- e.g. "Production", "Development"
    description     TEXT,
    created_at      TIMESTAMPTZ DEFAULT NOW(),
    UNIQUE (organization_id, name)          -- prevent duplicate group names per org
);

CREATE TABLE host_group_memberships (
    host_id         INTEGER NOT NULL REFERENCES hosts(id) ON DELETE CASCADE,
    host_group_id   INTEGER NOT NULL REFERENCES host_groups(id) ON DELETE CASCADE,
    PRIMARY KEY (host_id, host_group_id)    -- prevents duplicate memberships
);

-- Indexes for fast compliance lookups (NIST performance + audit)
CREATE INDEX idx_host_groups_org ON host_groups(organization_id);
CREATE INDEX idx_memberships_host ON host_group_memberships(host_id);
CREATE INDEX idx_memberships_group ON host_group_memberships(host_group_id);