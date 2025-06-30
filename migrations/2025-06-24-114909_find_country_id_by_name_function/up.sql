CREATE OR REPLACE FUNCTION find_country_id_by_name(
    name_query TEXT
)
RETURNS INTEGER AS $$
DECLARE
    country_id INTEGER;
BEGIN
    -- Case-insensitive exact match using ILIKE
    SELECT countries.gid
    INTO country_id
    FROM countries
    WHERE countries.name ILIKE name_query
    LIMIT 1;

    -- If no exact match, do fuzzy match using pg_trgm % operator
    IF country_id IS NULL THEN
        SELECT countries.gid
        INTO country_id
        FROM countries
        WHERE countries.name % name_query
        ORDER BY similarity(countries.name, name_query) DESC
        LIMIT 1;
    END IF;

    -- Return the country id (or NULL if not found)
    RETURN country_id;
END;
$$ LANGUAGE plpgsql;
