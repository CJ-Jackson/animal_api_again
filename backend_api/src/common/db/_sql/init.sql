PRAGMA foreign_keys = ON;

CREATE TABLE animal
(
    id          INTEGER PRIMARY KEY AUTOINCREMENT,
    species     TEXT NOT NULL,
    description TEXT NOT NULL
);

INSERT INTO animal (species, description)
VALUES ('dog', 'Domestic Dog'),
       ('cat', 'House Cat'),
       ('lion', 'King of the Jungle'),
       ('tiger', 'The one with the stripes'),
       ('elephant', 'Has a trunk'),
       ('monkey', 'The one with the nose'),
       ('horse', 'The one with the tail'),
       ('zebra', 'The one with the stripes, also.'),
       ('giraffe', 'The one with the trunk, also.'),
       ('panda', 'The one with the nose, also.'),
       ('llama', 'The one with the tail, also.'),
       ('koala', 'Like a huggable Teddy Bear.');