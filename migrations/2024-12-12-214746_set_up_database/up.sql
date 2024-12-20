-- Enum Definitions
CREATE TYPE user_role AS ENUM ('Admin', 'User');
CREATE TYPE hackathon_category AS ENUM ('education', 'military', 'web3_0', 'cybersecurity');
CREATE TYPE type_media AS ENUM ('video', 'photo');

-- Function for Updated Timestamp
CREATE OR REPLACE FUNCTION update_timestamp()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = CURRENT_TIMESTAMP;
RETURN NEW;
END;
$$ LANGUAGE plpgsql;

-- Users Table
CREATE TABLE users (
                       id SERIAL PRIMARY KEY,
                       first_name VARCHAR(50) NOT NULL,
                       last_name VARCHAR(50) NOT NULL,
                       password_hash VARCHAR(255) NOT NULL,
                       phone VARCHAR(20) NOT NULL UNIQUE,
                       role user_role DEFAULT 'User',
                       created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                       updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Hackathon Table
CREATE TABLE hackathon (
                           id SERIAL PRIMARY KEY,
                           user_id INT NOT NULL REFERENCES users(id) ON DELETE CASCADE,
                           category hackathon_category NOT NULL,
                           created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                           updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_hackathon_updated_at
    BEFORE UPDATE ON hackathon
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- News Table
CREATE TABLE news (
                      id SERIAL PRIMARY KEY,
                      description TEXT NOT NULL,
                      preview_id INT,
                      header VARCHAR(255) NOT NULL,
                      created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                      updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_news_updated_at
    BEFORE UPDATE ON news
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- News Media Table
CREATE TABLE news_media (
                            id SERIAL PRIMARY KEY,
                            src_url TEXT NOT NULL,
                            news_id INT REFERENCES news(id) ON DELETE CASCADE,
                            type_media type_media NOT NULL,
                            position INT NOT NULL,
                            created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                            updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_news_media_updated_at
    BEFORE UPDATE ON news_media
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

-- Announcement Banner Table
CREATE TABLE announcement_banner (
                                     id SERIAL PRIMARY KEY,
                                     src_url TEXT NOT NULL,
                                     type_media type_media NOT NULL,
                                     description VARCHAR(255) NOT NULL,
                                     showing BOOLEAN NOT NULL DEFAULT FALSE,
                                     created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                     updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE TRIGGER update_announcement_banner_updated_at
    BEFORE UPDATE ON announcement_banner
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();
