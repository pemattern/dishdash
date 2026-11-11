CREATE TABLE recipe_actions (
    code TEXT PRIMARY KEY
);

CREATE TABLE recipe_action_translations (
    recipe_action_code TEXT NOT NULL REFERENCES recipe_actions(code),
    locale_code TEXT NOT NULL REFERENCES locales(code),
    instructions TEXT NOT NULL,
    UNIQUE (recipe_action_code, locale_code)
);

CREATE TABLE recipe_steps (
    id UUID PRIMARY KEY DEFAULT uuid_v7(),
    recipe_id UUID NOT NULL REFERENCES recipes(id),
    recipe_action_code TEXT NOT NULL REFERENCES recipe_actions(code),
    position INT NOT NULL CHECK (position >= 0),
    parameters JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    CONSTRAINT recipe_id_position_unique
        UNIQUE (recipe_id, position)
        DEFERRABLE INITIALLY DEFERRED
);
