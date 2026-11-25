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

CREATE TYPE locale_code AS ENUM (
    'de',
    'en',
    'es',
    'fr',
    'it',
    'pt-BR'
);

CREATE TYPE physical_quantity_code AS ENUM (
    'count',
    'mass',
    'volume'
);

CREATE TYPE unit_system_code AS ENUM (
    'imperial',
    'metric',
    'us-customary'
);

CREATE TYPE unit_code AS ENUM (
    'piece',
    'dozen',
    'milligram',
    'gram',
    'kilogram',
    'ounce',
    'pound',
    'milliliter',
    'liter',
    'teaspoon',
    'tablespoon',
    'fluid-ounce',
    'cup',
    'pint',
    'quart',
    'gallon'
);

CREATE TABLE units (
    code unit_code NOT NULL,
    physical_quantity_code physical_quantity_code NOT NULL,
    unit_system_code unit_system_code NOT NULL,
    symbol TEXT NOT NULL,
    si_factor FLOAT8 NOT NULL CHECK (si_factor > 0),
    PRIMARY KEY (code, unit_system_code)
);

CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    name TEXT NOT NULL,
    email TEXT UNIQUE NOT NULL,
    password_hash TEXT NOT NULL,
    avatar_url TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_users_updated_at
    BEFORE UPDATE ON users
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();

INSERT INTO users (
    name,
    email,
    password_hash
) VALUES (
    'system',
    'system',
    -- password: secret
    '$argon2id$v=19$m=19456,t=2,p=1$c2FsdHltY3NhbHRzdGVy$AEBiLPbgT23PDqzPWAzKakRLOGlb+DaHevR0mzzMA/8'
);

CREATE TABLE user_preferences (
    user_id UUID PRIMARY KEY REFERENCES users(id) ON DELETE CASCADE,
    locale_code locale_code NOT NULL,
    unit_system_code unit_system_code NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY DEFAULT uuidv7(),
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    parent_id UUID REFERENCES refresh_tokens(id),
    opaque_token_hash TEXT UNIQUE NOT NULL,
    is_used BOOLEAN NOT NULL DEFAULT FALSE,
    expires_at TIMESTAMPTZ NOT NULL DEFAULT (NOW() + INTERVAL '2 months'),
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER update_refresh_tokens_updated_at
    BEFORE UPDATE ON refresh_tokens
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
