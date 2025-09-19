-- Your SQL goes here

CREATE TABLE user_addresses (
    id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    state VARCHAR(255),
    city VARCHAR(255),
    pincode VARCHAR(255),
    house_no VARCHAR(255) ,
    area VARCHAR(255)
);
