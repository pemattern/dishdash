CREATE TABLE physical_quantities (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT UNIQUE NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_physical_quantities_updated_at
    BEFORE UPDATE ON physical_quantities
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

INSERT INTO physical_quantities (
    name
) VALUES (
    'mass'
);

INSERT INTO physical_quantities (
    name
) VALUES (
    'volume'
);

INSERT INTO physical_quantities (
    name
) VALUES (
    'amount'
);
