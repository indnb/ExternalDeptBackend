-- Enum Definitions
CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE hackathon_category_2025 AS ENUM ('software', 'iot', 'gamedev', 'blockchain');
CREATE TYPE type_media AS ENUM ('video', 'photo');

-- Function for Updated Timestamp
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- University Table
CREATE TABLE hackathon_university_2025
(
    id         SERIAL PRIMARY KEY,
    name       TEXT NOT NULL UNIQUE,
    name_eng   TEXT NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_hackathon_university_updated_at
    BEFORE UPDATE
    ON hackathon_university_2025
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Hackathon Team Table
CREATE TABLE hackathon_team_2025
(
    id                    SERIAL PRIMARY KEY,
    name                  VARCHAR(255)            NOT NULL UNIQUE,
    category              hackathon_category_2025 NOT NULL,
    count_members         INT                     NOT NULL DEFAULT 0,
    created_at            TIMESTAMP                        DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP                        DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_hackathon_team_updated_at
    BEFORE UPDATE
    ON hackathon_team_2025
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Users Table
CREATE TABLE hackathon_user_2025
(
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(50)  NOT NULL,
    last_name     VARCHAR(50)  NOT NULL,
    phone         VARCHAR(20)  DEFAULT NULL UNIQUE,
    nickname_tg   VARCHAR(255) DEFAULT NULL UNIQUE,
    university_id INT NOT NULL,
    team_id       INT NOT NULL,
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_university FOREIGN KEY (university_id) REFERENCES hackathon_university_2025 (id) ON DELETE SET NULL,
    CONSTRAINT fk_team FOREIGN KEY (team_id) REFERENCES hackathon_team_2025 (id) ON DELETE SET NULL
);

-- Captain Table
CREATE TABLE hackathon_team_captain_2025 (
    team_id INT PRIMARY KEY,
    captain_id INT UNIQUE NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_team
        FOREIGN KEY (team_id)
        REFERENCES hackathon_team_2025 (id)
        ON DELETE CASCADE,
    CONSTRAINT fk_captain
        FOREIGN KEY (captain_id)
        REFERENCES hackathon_user_2025 (id)
        ON DELETE CASCADE
);

-- Trigger to Update Timestamp
CREATE TRIGGER update_hackathon_user_updated_at
    BEFORE UPDATE
    ON hackathon_user_2025
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Trigger to Increment Team Member Count
CREATE OR REPLACE FUNCTION increment_team_member_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE hackathon_team_2025
    SET count_members = count_members + 1
    WHERE id = NEW.team_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Trigger to Increment Team Member Count on Insert
CREATE TRIGGER increment_team_member_trigger
    AFTER INSERT
    ON hackathon_user_2025
    FOR EACH ROW
    EXECUTE FUNCTION increment_team_member_count();

-- Trigger to Decrement Team Member Count
CREATE OR REPLACE FUNCTION decrement_team_member_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE hackathon_team_2025
    SET count_members = count_members - 1
    WHERE id = OLD.team_id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

-- Trigger to Decrement Team Member Count on Delete or Update
CREATE TRIGGER decrement_team_member_trigger
    AFTER DELETE OR UPDATE OF team_id
    ON hackathon_user_2025
    FOR EACH ROW
    WHEN (OLD.team_id IS NOT NULL)
    EXECUTE FUNCTION decrement_team_member_count();

-- Optional: Create a trigger to update the updated_at timestamp on changes
CREATE TRIGGER update_hackathon_team_captain_updated_at
    BEFORE UPDATE
    ON hackathon_team_captain_2025
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
