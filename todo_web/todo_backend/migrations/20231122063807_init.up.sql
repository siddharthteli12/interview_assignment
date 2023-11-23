-- Add up migration script here

CREATE TABLE users (
        user_mail VARCHAR(50) PRIMARY KEY NOT NULL,
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
        user_mail VARCHAR(50) NOT NULL,
        FOREIGN KEY(user_mail) REFERENCES users(user_mail),
        created_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW(),
            updated_at TIMESTAMP
        WITH
            TIME ZONE DEFAULT NOW()
    );
