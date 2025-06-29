CREATE INDEX IF NOT EXISTS cities_geom_gist_idx ON cities USING GIST ("geom");
CREATE INDEX IF NOT EXISTS cities_name_trgm_idx ON cities USING gin (name gin_trgm_ops);