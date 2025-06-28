CREATE INDEX countries_name_trgm_idx ON countries USING gin (name gin_trgm_ops);

CREATE OR REPLACE FUNCTION find_country_geometry_by_name(
    country_query TEXT
)
RETURNS GEOMETRY AS $$
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

    -- Return the country geometry (or NULL if not found)
    RETURN country_geom;
END;
$$ LANGUAGE plpgsql;
