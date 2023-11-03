DELIMITER //

CREATE FUNCTION generate_random_base64_10char() RETURNS VARCHAR(10) DETERMINISTIC
BEGIN
    DECLARE random_base64 VARCHAR(10);
    
    SET random_base64 = CONCAT(
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1),
        SUBSTRING('ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789-_', FLOOR(RAND() * 64) + 1, 1)
    );

    RETURN random_base64;
END //

DELIMITER ;


-- Modify the 'users' table
CREATE TABLE users (
    unique_id VARCHAR(10) PRIMARY KEY,
    email VARCHAR(255) UNIQUE NOT NULL,
    username VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(127) NOT NULL,
    date_of_registration BIGINT NOT NULL,
    country VARCHAR(3)
);

DELIMITER //
CREATE FUNCTION generate_user_id() RETURNS VARCHAR(10) DETERMINISTIC
BEGIN
    DECLARE collision_count INT DEFAULT 0;
    DECLARE new_id VARCHAR(10);
    SET new_id = generate_random_base64_10char();
    -- Check for collisions and regenerate the unique_id if needed
    SELECT COUNT(*) INTO collision_count FROM users WHERE unique_id = new_id;
    WHILE collision_count > 0 DO
        SET new_id = generate_random_base64_10char();
        SELECT COUNT(*) INTO collision_count FROM users WHERE unique_id = new_id;
    END WHILE;

    RETURN new_id;
END //
DELIMITER ;


-- Modify the 'stacks' table
CREATE TABLE stacks (
    unique_id VARCHAR(10) PRIMARY KEY,
    owner_id VARCHAR(10) NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES users(unique_id) ON DELETE CASCADE,
    name VARCHAR(255) NOT NULL,
    visibility BOOLEAN NOT NULL,
    cards_count INT NOT NULL DEFAULT 0,
    tags VARCHAR(255) NOT NULL
);

DELIMITER //
CREATE FUNCTION generate_stack_id() RETURNS VARCHAR(10) DETERMINISTIC
BEGIN
    DECLARE collision_count INT DEFAULT 0;
    DECLARE new_id VARCHAR(10);
    SET new_id = generate_random_base64_10char();
    -- Check for collisions and regenerate the unique_id if needed
    SELECT COUNT(*) INTO collision_count FROM stacks WHERE unique_id = new_id;
    WHILE collision_count > 0 DO
        SET new_id = generate_random_base64_10char();
        SELECT COUNT(*) INTO collision_count FROM stacks WHERE unique_id = new_id;
    END WHILE;

    RETURN new_id;
END //
DELIMITER ;


-- Modify the 'cards' table
CREATE TABLE cards (
    unique_id VARCHAR(10) PRIMARY KEY,
    stack_id VARCHAR(10) NOT NULL,
    FOREIGN KEY (stack_id) REFERENCES stacks(unique_id) ON DELETE CASCADE,
    frontside TEXT NOT NULL,
    backside TEXT NOT NULL
);

DELIMITER //
CREATE FUNCTION generate_card_id() RETURNS VARCHAR(10) DETERMINISTIC
BEGIN
    DECLARE collision_count INT DEFAULT 0;
    DECLARE new_id VARCHAR(10);
    SET new_id = generate_random_base64_10char();
    -- Check for collisions and regenerate the unique_id if needed
    SELECT COUNT(*) INTO collision_count FROM cards WHERE unique_id = new_id;
    WHILE collision_count > 0 DO
        SET new_id = generate_random_base64_10char();
        SELECT COUNT(*) INTO collision_count FROM cards WHERE unique_id = new_id;
    END WHILE;

    RETURN new_id;
END //
DELIMITER ;


DELIMITER //

CREATE TRIGGER update_cards_count_after_insert
AFTER INSERT ON cards
FOR EACH ROW
BEGIN
    UPDATE stacks
    SET cards_count = cards_count + 1
    WHERE unique_id = NEW.stack_id;
END//

DELIMITER ;

DELIMITER //

CREATE TRIGGER update_cards_count_after_delete
AFTER DELETE ON cards
FOR EACH ROW
BEGIN
    UPDATE stacks
    SET cards_count = cards_count - 1
    WHERE unique_id = OLD.stack_id;
END//

DELIMITER ;