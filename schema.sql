DROP TABLE transactions;
DROP TABLE balances;
DROP TABLE payments;
DROP TABLE group_participants;
DROP TABLE groups;
DROP TABLE users;

CREATE TABLE users (
    user_id SERIAL PRIMARY KEY,
    display_name VARCHAR(100) NOT NULL,
    email VARCHAR(255) NOT NULL UNIQUE,
    img VARCHAR(255),
    date_created INT NOT NULL
);

CREATE TABLE groups (
    group_id SERIAL PRIMARY KEY,
    group_name VARCHAR(100) NOT NULL,
    /* group participants */
    img VARCHAR(255)
);

CREATE TABLE group_participants (
    user_id INT,
    group_id INT,
    PRIMARY KEY (user_id, group_id),
    FOREIGN KEY (user_id) REFERENCES users(user_id),
    FOREIGN KEY (group_id) REFERENCES groups(group_id)
);

CREATE TABLE payments (
    pmt_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    group_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(group_id) ON DELETE CASCADE,
    amt DECIMAL(10, 2) NOT NULL,
    date_created INT NOT NULL,
    description VARCHAR(100)
);

CREATE TABLE balances (
    balance_id SERIAL PRIMARY KEY,
    user_id INT NOT NULL,
    group_id INT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(group_id) ON DELETE CASCADE,
    amt DECIMAL(10, 2) NOT NULL,
    UNIQUE (user_id, group_id)
);

CREATE TABLE transactions (
    t_id SERIAL PRIMARY KEY,
    ower_id INT NOT NULL,
    owed_id INT NOT NULL,
    group_id INT NOT NULL,
    FOREIGN KEY (ower_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (owed_id) REFERENCES users(user_id) ON DELETE CASCADE,
    FOREIGN KEY (group_id) REFERENCES groups(group_id) ON DELETE CASCADE,
    amt DECIMAL(10, 2) NOT NULL,
    UNIQUE (ower_id, owed_id, group_id)
);

INSERT INTO users (display_name, email, img, date_created)
VALUES
    ('Aiden', 'aseibel@trinity.edu', 'https://media.licdn.com/dms/image/v2/D5603AQHsygk4iYrt2w/profile-displayphoto-shrink_800_800/profile-displayphoto-shrink_800_800/0/1677866509923?e=1735171200&v=beta&t=Ll3SDsXMp0TB7qBRk1K104tQhLwnZ7h_aBV3W8thX4o', 1729925633),
    ('Khoi', 'ktran1@trinity.edu', 'https://thumbs.dreamstime.com/b/cute-doggy-3257704.jpg', 1730012050),
    ('Sam', 'scohen@trinity.edu', 'https://media.licdn.com/dms/image/v2/D5603AQG6Ueqp8I6Ksg/profile-displayphoto-shrink_400_400/profile-displayphoto-shrink_400_400/0/1728070686967?e=1735171200&v=beta&t=tyjojF9DdzPwB3AIARKHe8NmD-k7QkhtptS7g7mI0QE', 1729925633),
    ('Test User 1', 'test1@gmail.com', '', 1729925633),
    ('Test User 2', 'test2@yahoo.com', '', 1729925633);

INSERT INTO groups (group_name, img)
VALUES
    ('RowdyHacks', 'rowdyhacks'),
    ('Roommates', 'https://i0.wp.com/www.michigandaily.com/wp-content/uploads/2022/02/online_jeh.opinion.Dormroom.02.22.22.0105.jpg'),
    ('Pitbull Concert', 'https://mesquite-news.com/wp-content/uploads/2022/09/pitbull_090922_xg_sm-scaled.jpg'),
    ('Project Car', '');

INSERT INTO group_participants (user_id, group_id)
VALUES
    (1, 1),
    (2, 1),
    (3, 1),
    (1, 2),
    (2, 2),
    (1, 3),
    (2, 3),
    (3, 3),
    (4, 3),
    (5, 3),
    (4, 4),
    (5, 4);

INSERT INTO payments (user_id, group_id, amt, date_created, description)
VALUES
    (1, 1, 5.00, 1729925633, 'a little snack'),
    (1, 2, 10.00, 1729925633, 'grocery run'),
    (1, 3, 15.00, 1718825633, 'something special'),
    (2, 1, 8.00, 1729925633, 'gas'),
    (3, 1, 100.00, 1729925633, 'feeling generous')
    (4, 5, 200.00, 1718825633, 'for the car'),
    (5, 4, 17.17, 1718843333, 'oil change');

INSERT INTO balances (user_id, group_id, amt)
VALUES
    (1, 1, 20.00),
    (2, 1, 15.00),
    (3, 1, 8.00),
    (1, 2, 0.00),
    (2, 2, 20.00),
    (1, 3, 0.00),
    (2, 3, 0.00),
    (3, 3, 15.50),
    (4, 3, 20.00),
    (5, 3, 25.00),
    (4, 4, 100.00),
    (5, 4, 150.00);

INSERT INTO transactions (ower_id, owed_id, group_id, amt)
VALUES
    (1, 2, 1, 10.00),
    (1, 3, 1, 10.00),
    (2, 1, 1, 7.50),
    (2, 3, 1, 7.50),
    (3, 1, 1, 4.00),
    (3, 2, 1, 4.00),
    (1, 2, 2, 0.00),
    (2, 1, 2, 20.00),
    (1, 2, 3, 0.00),
    (1, 3, 3, 0.00),
    (1, 4, 3, 0.00),
    (1, 5, 3, 0.00),
    (2, 1, 3, 0.00),
    (2, 3, 3, 0.00),
    (2, 4, 3, 0.00),
    (2, 5, 3, 0.00),
    (3, 1, 3, 3.87),
    (3, 2, 3, 3.87),
    (3, 4, 3, 3.87),
    (3, 5, 3, 3.87),
    (4, 1, 3, 5.00),
    (4, 2, 3, 5.00),
    (4, 3, 3, 5.00),
    (4, 5, 3, 5.00),
    (5, 1, 3, 6.25),
    (5, 2, 3, 6.25),
    (5, 3, 3, 6.25),
    (5, 4, 3, 6.25),
    (4, 5, 4, 100.0),
    (5, 4, 4, 150.00);
    