CREATE OR REPLACE FUNCTION increment_team_member_count() RETURNS TRIGGER AS $$
BEGIN
UPDATE hackathon_team_2024
SET count_members = count_members + 1
WHERE id = NEW.team_id;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER increment_team_member_trigger
    AFTER INSERT ON hackathon_user_2024
    FOR EACH ROW
    EXECUTE FUNCTION increment_team_member_count();
