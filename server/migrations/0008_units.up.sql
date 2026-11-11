CREATE TABLE units (
    code TEXT PRIMARY KEY,
    physical_quantity_code TEXT NOT NULL REFERENCES physical_quantities(code) ON DELETE RESTRICT,
    unit_system_code TEXT NOT NULL REFERENCES unit_systems(code) ON DELETE RESTRICT,
    symbol TEXT NOT NULL,
    si_factor FLOAT8 NOT NULL CHECK (si_factor > 0)
);
