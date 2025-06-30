CREATE OR REPLACE FUNCTION set_photo_city_from_gps()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT' OR (TG_OP = 'UPDATE' AND (OLD.gps_location IS DISTINCT FROM NEW.gps_location)))
       AND NEW.gps_location IS NOT NULL THEN
        NEW.city_id := find_city_id_by_geom(NEW.gps_location, 10000);
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_photo_city_trigger
    BEFORE INSERT OR UPDATE ON photos
    FOR EACH ROW
    EXECUTE FUNCTION set_photo_city_from_gps();
