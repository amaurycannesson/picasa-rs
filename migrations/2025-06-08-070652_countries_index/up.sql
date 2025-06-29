CREATE INDEX countries_name_trgm_idx ON countries USING gin (name gin_trgm_ops);
