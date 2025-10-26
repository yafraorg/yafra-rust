-- Create the business database
CREATE DATABASE IF NOT EXISTS business;
USE business;

-- Create the transactions table
CREATE TABLE IF NOT EXISTS transactions (
    id INT AUTO_INCREMENT PRIMARY KEY,
    purpose VARCHAR(255),
    amount DECIMAL(10, 2),
    currency ENUM('USD', 'EUR', 'GBP'),
    date DATE,
    customer VARCHAR(255),
    item VARCHAR(255),
    status ENUM('pending', 'completed', 'canceled')
);

-- Populate the transactions table with 3000 dummy entries
DELIMITER $$
CREATE PROCEDURE populate_transactions()
BEGIN
  DECLARE i INT DEFAULT 1;
  WHILE i <= 3000 DO
    INSERT INTO transactions (purpose, amount, currency, date, customer, item, status)
    VALUES (
      CONCAT('Purpose ', i), -- Replace with actual purpose data or leave as is
      RAND() * 1000.00, -- Random value between 0 and 1000
      CASE FLOOR(RAND() * 3) WHEN 0 THEN 'USD' WHEN 1 THEN 'EUR' ELSE 'GBP' END, -- Random currency (USD, EUR, GBP)
      DATE_ADD(CURDATE(), INTERVAL FLOOR(RAND() * -365) DAY), -- Random date within the past year
      CONCAT('Customer ', i), -- Replace with actual customer data or leave as is
      CONCAT('Item ', i), -- Replace with actual item data or leave as is
      CASE FLOOR(RAND() * 3) WHEN 0 THEN 'pending' WHEN 1 THEN 'completed' ELSE 'canceled' END -- Random status (pending, completed, canceled)
    );
    SET i = i + 1;
  END WHILE;
END$$
DELIMITER ;

-- Call the procedure to populate the transactions table
CALL populate_transactions();