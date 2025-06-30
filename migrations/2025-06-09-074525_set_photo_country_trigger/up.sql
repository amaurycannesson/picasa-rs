CREATE OR REPLACE FUNCTION set_photo_country_from_gps()
RETURNS TRIGGER AS $$
BEGIN
    IF (TG_OP = 'INSERT' OR (TG_OP = 'UPDATE' AND (OLD.gps_location IS DISTINCT FROM NEW.gps_location))) 
       AND NEW.gps_location IS NOT NULL THEN
        NEW.country_id := find_country_id_by_geom(NEW.gps_location);
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER set_photo_country_trigger
    BEFORE INSERT OR UPDATE ON photos
    FOR EACH ROW
    EXECUTE FUNCTION set_photo_country_from_gps();
