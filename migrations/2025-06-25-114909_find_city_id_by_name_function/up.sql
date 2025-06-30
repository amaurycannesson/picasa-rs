CREATE OR REPLACE FUNCTION find_city_id_by_name(
    name_query TEXT
)
RETURNS INTEGER AS $$
DECLARE
    city_id INTEGER;
BEGIN
    -- Case-insensitive exact match using ILIKE
    SELECT cities.geonameid
    INTO city_id
    FROM cities
    WHERE cities.name ILIKE name_query
    ORDER BY cities.population DESC
    LIMIT 1;

    -- If no exact match, do fuzzy match using pg_trgm % operator
    IF city_id IS NULL THEN
        SELECT cities.geonameid
        INTO city_id
        FROM cities
        WHERE cities.name % name_query
        ORDER BY cities.population DESC, similarity(cities.name, name_query) DESC
        LIMIT 1;
    END IF;

    -- Return the city id (or NULL if not found)
    RETURN city_id;
END;
$$ LANGUAGE plpgsql;
