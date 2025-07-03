CREATE TRIGGER update_people_updated_at_trigger
    BEFORE UPDATE ON people 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();
