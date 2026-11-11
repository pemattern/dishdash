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
