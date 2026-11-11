CREATE EXTENSION pg_trgm;
CREATE EXTENSION unaccent;

CREATE FUNCTION search_normalize(input TEXT)
RETURNS TEXT
LANGUAGE SQL
STABLE
PARALLEL SAFE
AS $$
    SELECT lower(unaccent(normalize(input, NFKC)));
$$;

CREATE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

CREATE TABLE seed_data_imports (
    file_name TEXT PRIMARY KEY,
    checksum BYTEA NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_seed_data_imports_updated_at
    BEFORE UPDATE ON seed_data_imports
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
