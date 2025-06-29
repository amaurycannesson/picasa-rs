CREATE OR REPLACE FUNCTION find_city_geometry_by_name(
    city_query TEXT
)
RETURNS GEOMETRY AS $$
DECLARE
    city_geom GEOMETRY;
BEGIN
    -- Case-insensitive exact match using ILIKE
    SELECT
        cities.geom
    INTO city_geom
    FROM cities
    WHERE cities.name ILIKE city_query
    ORDER BY cities.population DESC
    LIMIT 1;

    -- If no exact match, do fuzzy match using pg_trgm % operator
    IF city_geom IS NULL THEN
        SELECT
            cities.geom
        INTO city_geom
        FROM cities
        WHERE cities.name % city_query
        ORDER BY cities.population DESC, similarity(cities.name, city_query) DESC
        LIMIT 1;
    END IF;

    -- Return the city geometry (or NULL if not found)
    RETURN city_geom;
END;
$$ LANGUAGE plpgsql;
