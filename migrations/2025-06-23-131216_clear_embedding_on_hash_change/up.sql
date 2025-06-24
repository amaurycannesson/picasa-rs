CREATE OR REPLACE FUNCTION clear_embedding_on_hash_change()
RETURNS TRIGGER AS $$
BEGIN
    IF OLD.hash IS NOT NULL AND 
       NEW.hash IS NOT NULL AND 
       OLD.hash IS DISTINCT FROM NEW.hash THEN
        NEW.embedding := NULL;
    END IF;
    
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER clear_embedding_on_hash_change_trigger
    BEFORE UPDATE ON photos
    FOR EACH ROW
    EXECUTE FUNCTION clear_embedding_on_hash_change();