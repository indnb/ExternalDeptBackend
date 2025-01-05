-- Create the new table for hackathon teams
CREATE TABLE hackathon_team_2024
(
    id                    SERIAL PRIMARY KEY,
    name                  VARCHAR(255)            NOT NULL,
    category              hackathon_category_2024 NOT NULL,
    password_registration VARCHAR(255)            NOT NULL,
    count_members         INT                     NOT NULL DEFAULT 0,
    created_at            TIMESTAMP                        DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP                        DEFAULT CURRENT_TIMESTAMP
);

-- Drop the old category column from hackathon_user_2024
ALTER TABLE hackathon_user_2024 DROP COLUMN category;

-- Add the new column in hackathon_user_2024 for team reference
ALTER TABLE hackathon_user_2024
    ADD COLUMN team_id INT;

-- Add foreign key constraint for the new team_id column
ALTER TABLE hackathon_user_2024
    ADD CONSTRAINT fk_team
        FOREIGN KEY (team_id) REFERENCES hackathon_team_2024 (id)
            ON DELETE SET NULL;

-- Trigger for updated timestamp in hackathon_team_2024
CREATE TRIGGER update_hackathon_team_2024_updated_at
    BEFORE UPDATE
    ON hackathon_team_2024
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Adjust related triggers and constraints if necessary
-- For example, ensure the updated_at trigger works for both old and new structures
CREATE
OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at
= CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$
LANGUAGE plpgsql;

-- Ensure all foreign key constraints and relationships are consistent
ALTER TABLE hackathon_user_2024
DROP
CONSTRAINT IF EXISTS fk_category;
