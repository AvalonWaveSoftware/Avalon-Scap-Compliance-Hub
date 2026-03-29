-- 000004__add_description_to_hosts.sql
-- Forward migration to match the seed data (NIST CM-3)

ALTER TABLE hosts ADD COLUMN description TEXT;