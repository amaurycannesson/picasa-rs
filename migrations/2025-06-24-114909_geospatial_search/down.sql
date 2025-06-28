DROP INDEX IF EXISTS countries_name_trgm_idx;

DROP FUNCTION IF EXISTS find_country_geometry_by_name(TEXT);
