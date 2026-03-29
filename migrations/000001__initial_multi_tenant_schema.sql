-- V1__initial_multi_tenant_schema.sql
-- NIST-compliant baseline tables for organizations, hosts, and scans

CREATE TABLE organizations (
    id              SERIAL PRIMARY KEY,
    name            TEXT NOT NULL UNIQUE,
    created_at      TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE hosts (
    id                  SERIAL PRIMARY KEY,
    organization_id     INTEGER NOT NULL REFERENCES organizations(id) ON DELETE CASCADE,
    hostname            TEXT NOT NULL,
    ip_address          TEXT,
    last_seen           TIMESTAMPTZ DEFAULT NOW(),
    created_at          TIMESTAMPTZ DEFAULT NOW()
);

CREATE TABLE scans (
    id                  SERIAL PRIMARY KEY,
    host_id             INTEGER NOT NULL REFERENCES hosts(id) ON DELETE CASCADE,
    arf_filename        TEXT NOT NULL,
    profile             TEXT,
    compliance_percent  NUMERIC(5,2),
    scanned_at          TIMESTAMPTZ DEFAULT NOW(),
    raw_arf_xml         TEXT   -- we store the raw file path, not the XML itself
);

-- Simple index for fast compliance lookups
CREATE INDEX idx_hosts_org ON hosts(organization_id);
CREATE INDEX idx_scans_host ON scans(host_id);