CREATE TABLE customer
(
    name TEXT PRIMARY KEY NOT NULL,
    age INTEGER,
    reg_date DATE NOT NULL 
);

INSERT INTO customer
    (name, age, reg_date)
 VALUES
    ('田中 一郎', 34, DATE '2021-03-13'),
    ('佐藤 二郎', NULL, DATE '2021-03-15'),
    ('鈴木 三郎', 43, DATE '2021-03-14');
