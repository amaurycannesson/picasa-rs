CREATE TRIGGER update_faces_updated_at_trigger
    BEFORE UPDATE ON faces 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
