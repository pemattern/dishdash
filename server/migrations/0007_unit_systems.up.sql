CREATE TABLE unit_systems (
    code TEXT PRIMARY KEY
);

INSERT INTO unit_systems (code)
VALUES
    ('metric'),
    ('imperial'),
    ('us_customary');
