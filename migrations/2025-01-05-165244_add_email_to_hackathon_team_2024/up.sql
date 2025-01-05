ALTER TABLE hackathon_team_2024
    ADD COLUMN email VARCHAR(255) NOT NULL;

ALTER TABLE hackathon_team_2024
    ADD CONSTRAINT unique_email UNIQUE (email);
