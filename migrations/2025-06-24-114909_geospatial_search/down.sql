DROP INDEX IF EXISTS countries_name_trgm_idx;

DROP FUNCTION IF EXISTS find_photos_by_country(TEXT);