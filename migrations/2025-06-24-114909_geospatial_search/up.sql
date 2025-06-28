CREATE INDEX countries_name_trgm_idx ON countries USING gin (name gin_trgm_ops);

CREATE OR REPLACE FUNCTION find_photos_by_country(
    country_query TEXT
)
RETURNS SETOF photos AS $$
DECLARE
    country_geom GEOMETRY;
BEGIN
    -- Case-insensitive exact match using ILIKE
    SELECT
        countries.geom
    INTO country_geom
    FROM countries
    WHERE countries.name ILIKE country_query
    LIMIT 1;

    -- If no exact match, do fuzzy match using pg_trgm % operator
    IF country_geom IS NULL THEN
        SELECT
            countries.geom
        INTO country_geom
        FROM countries
        WHERE countries.name % country_query
        ORDER BY similarity(countries.name, country_query) DESC
        LIMIT 1;
    END IF;

    -- Return photos if we found a match
    IF country_geom IS NOT NULL THEN
        RETURN QUERY
        SELECT *
        FROM photos
        WHERE ST_Within(photos.gps_location, country_geom);
    END IF;
END;
$$ LANGUAGE plpgsql;