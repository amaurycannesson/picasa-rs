CREATE OR REPLACE FUNCTION find_country_id_by_geom(
    geom_query GEOMETRY
)
RETURNS INTEGER AS $$
DECLARE
    country_id INTEGER;
BEGIN
    SELECT countries.gid
    INTO country_id
    FROM countries
    WHERE ST_Within(geom_query, countries.geom)
    LIMIT 1;
    
    RETURN country_id;
END;
$$ LANGUAGE plpgsql;
