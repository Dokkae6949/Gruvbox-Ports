-- Add trigram index for fuzzy search on name only
CREATE INDEX idx_ports_name_trgm ON ports USING GIN (name gin_trgm_ops);
