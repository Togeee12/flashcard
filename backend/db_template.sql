-- Create the 'users' table
CREATE TABLE users (
    id INT PRIMARY KEY,
    email VARCHAR(255) NOT NULL,
    username VARCHAR(255) NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    country VARCHAR(255)
);

-- Create the 'stacks' table
CREATE TABLE stacks (
    id INT PRIMARY KEY,
    owner_id INT NOT NULL,
    visibility VARCHAR(255) NOT NULL,
    tags VARCHAR(255)
);

-- Create the 'cards' table
CREATE TABLE cards (
    id INT PRIMARY KEY,
    stack_id INT NOT NULL,
    frontside TEXT NOT NULL,
    backside TEXT NOT NULL
);