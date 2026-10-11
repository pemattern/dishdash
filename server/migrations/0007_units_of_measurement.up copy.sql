CREATE TABLE units (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    physical_quantity_id UUID NOT NULL REFERENCES physical_quantities(id) ON DELETE CASCADE,
    name TEXT UNIQUE NOT NULL,
    symbol TEXT NOT NULL,
    si_factor NUMERIC NOT NULL CHECK (si_factor > 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_units_updated_at
    BEFORE UPDATE ON units
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

INSERT INTO units (name, symbol, physical_quantity_id, to_si_factor)
    SELECT 'gram', 'g', pq.id, 0.001
    FROM physical_quantities pq
    WHERE pq.name = 'mass';
