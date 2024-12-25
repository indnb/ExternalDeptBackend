ALTER TABLE users RENAME TO hackathon_user_2024;
ALTER TABLE hackathon_user_2024 ADD COLUMN category hackathon_category NOT NULL;
DROP TABLE hackathon;
ALTER TYPE hackathon_category RENAME TO hackathon_category_2024;
CREATE TABLE hackathon_university_2024 (
                                           id SERIAL PRIMARY KEY,
                                           name VARCHAR(255) NOT NULL UNIQUE,
                                           created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
                                           updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
CREATE TRIGGER update_hackathon_university_2024_updated_at
    BEFORE UPDATE ON hackathon_university_2024
    FOR EACH ROW
    EXECUTE FUNCTION update_timestamp();

ALTER TABLE hackathon_user_2024
    ADD COLUMN university INT;

ALTER TABLE hackathon_user_2024
    ADD CONSTRAINT fk_university
        FOREIGN KEY (university) REFERENCES hackathon_university_2024(id)
            ON DELETE SET NULL;

ALTER TABLE hackathon_user_2024 DROP COLUMN password_hash;
ALTER TABLE hackathon_user_2024 DROP COLUMN role;
