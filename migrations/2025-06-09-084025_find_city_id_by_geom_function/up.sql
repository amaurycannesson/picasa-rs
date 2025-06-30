CREATE OR REPLACE FUNCTION find_city_id_by_geom(
    geom_query GEOMETRY,
    radius NUMERIC
)
RETURNS INTEGER AS $$
DECLARE
    city_id INTEGER;
BEGIN
    SELECT cities.geonameid
    INTO city_id
    FROM cities
    WHERE ST_DWithin(
        ST_Transform(geom_query, 3857), 
        ST_Transform(cities.geom, 3857),
        radius
    )
    ORDER BY cities.population DESC
    LIMIT 1;
    
    RETURN city_id;
END;
$$ LANGUAGE plpgsql;
