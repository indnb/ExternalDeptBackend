-- Enum Definitions
CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE hackathon_category_2024 AS ENUM ('education', 'military', 'web3_0', 'cybersecurity');
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
CREATE TABLE hackathon_university_2024
(
    id         SERIAL PRIMARY KEY,
    name       VARCHAR(255) NOT NULL UNIQUE,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_hackathon_university_updated_at
    BEFORE UPDATE
    ON hackathon_university_2024
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Hackathon Team Table
CREATE TABLE hackathon_team_2024
(
    id                    SERIAL PRIMARY KEY,
    name                  VARCHAR(255)            NOT NULL,
    category              hackathon_category_2024 NOT NULL,
    password_registration VARCHAR(255)            NOT NULL,
    count_members         INT                     NOT NULL DEFAULT 0,
    nickname_tg           VARCHAR(255)            NOT NULL UNIQUE,
    created_at            TIMESTAMP                        DEFAULT CURRENT_TIMESTAMP,
    updated_at            TIMESTAMP                        DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_hackathon_team_updated_at
    BEFORE UPDATE
    ON hackathon_team_2024
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Users Table
CREATE TABLE hackathon_user_2024
(
    id            SERIAL PRIMARY KEY,
    first_name    VARCHAR(50)  NOT NULL,
    last_name     VARCHAR(50)  NOT NULL,
    phone         VARCHAR(20)  NOT NULL UNIQUE,
    nickname_tg   VARCHAR(255) NOT NULL UNIQUE,
    university_id INT,
    team_id       INT,
    created_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at    TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT fk_university FOREIGN KEY (university_id) REFERENCES hackathon_university_2024 (id) ON DELETE SET NULL,
    CONSTRAINT fk_team FOREIGN KEY (team_id) REFERENCES hackathon_team_2024 (id) ON DELETE SET NULL
);

CREATE TRIGGER update_hackathon_user_updated_at
    BEFORE UPDATE
    ON hackathon_user_2024
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Trigger to Increment Team Member Count
CREATE OR REPLACE FUNCTION increment_team_member_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE hackathon_team_2024
    SET count_members = count_members + 1
    WHERE id = NEW.team_id;
    RETURN NEW;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER increment_team_member_trigger
    AFTER INSERT
    ON hackathon_user_2024
    FOR EACH ROW
    EXECUTE FUNCTION increment_team_member_count();

-- Trigger to Decrement Team Member Count
CREATE OR REPLACE FUNCTION decrement_team_member_count()
RETURNS TRIGGER AS $$
BEGIN
    UPDATE hackathon_team_2024
    SET count_members = count_members - 1
    WHERE id = OLD.team_id;
    RETURN OLD;
END;
$$ LANGUAGE plpgsql;

CREATE TRIGGER decrement_team_member_trigger
    AFTER DELETE OR UPDATE OF team_id
    ON hackathon_user_2024
    FOR EACH ROW
    WHEN (OLD.team_id IS NOT NULL)
    EXECUTE FUNCTION decrement_team_member_count();

-- News Table
CREATE TABLE news
(
    id          SERIAL PRIMARY KEY,
    description TEXT         NOT NULL,
    preview_id  INT,
    header      VARCHAR(255) NOT NULL UNIQUE,
    created_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_news_updated_at
    BEFORE UPDATE
    ON news
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- News Media Table
CREATE TABLE news_media
(
    id         SERIAL PRIMARY KEY,
    src_url    TEXT       NOT NULL,
    news_id    INT REFERENCES news (id) ON DELETE CASCADE,
    type_media type_media NOT NULL,
    position   INT        NOT NULL,
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    CONSTRAINT unique_position_per_news UNIQUE (news_id, position)
);

CREATE TRIGGER update_news_media_updated_at
    BEFORE UPDATE
    ON news_media
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Announcement Banner Table
CREATE TABLE announcement_banner
(
    id          SERIAL PRIMARY KEY,
    src_url     TEXT         NOT NULL,
    type_media  type_media   NOT NULL DEFAULT 'photo',
    description VARCHAR(255) NOT NULL,
    showing     BOOLEAN      NOT NULL DEFAULT FALSE,
    created_at  TIMESTAMP             DEFAULT CURRENT_TIMESTAMP,
    updated_at  TIMESTAMP             DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_announcement_banner_updated_at
    BEFORE UPDATE
    ON announcement_banner
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

