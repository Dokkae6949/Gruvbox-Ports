-- Enable PostgreSQL extensions for fuzzy search
CREATE EXTENSION IF NOT EXISTS pg_trgm;      -- Trigram matching for fuzzy search
CREATE EXTENSION IF NOT EXISTS fuzzystrmatch; -- Levenshtein distance and other fuzzy matching
