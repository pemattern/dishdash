CREATE TABLE unit_systems (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_unit_systems_updated_at
    BEFORE UPDATE ON unit_systems
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

INSERT INTO unit_systems (id, name)
VALUES
    ('metric'),
    ('imperial'),
    ('us_customary');
