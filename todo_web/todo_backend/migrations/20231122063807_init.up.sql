-- Add up migration script here

CREATE TABLE users (
        user_id SERIAL PRIMARY KEY NOT NULL,
        first_name VARCHAR(50) NOT NULL,
        second_name VARCHAR(50) NOT NULL,
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );


CREATE TABLE
    IF NOT EXISTS tasks (
        task_id SERIAL PRIMARY KEY NOT NULL,
        task_description VARCHAR(255) NOT NULL,
        user_id SERIAL,
        FOREIGN KEY(user_id) REFERENCES users(user_id),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
