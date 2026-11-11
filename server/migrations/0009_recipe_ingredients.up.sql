CREATE TABLE recipe_ingredients (
    recipe_id UUID NOT NULL REFERENCES recipes(id) ON DELETE CASCADE,
    ingredient_code TEXT NOT NULL REFERENCES ingredients(code) ON DELETE RESTRICT,
    unit_code TEXT NOT NULL REFERENCES units(code) ON DELETE RESTRICT,
    position INT NOT NULL CHECK (position >= 0),
    quantity REAL NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),

    PRIMARY KEY (recipe_id, ingredient_code),
    CONSTRAINT recipe_ingredient_recipe_id_position_unique
        UNIQUE (recipe_id, position)
        DEFERRABLE INITIALLY DEFERRED
);

CREATE TRIGGER update_recipe_ingredients_updated_at
    BEFORE UPDATE ON recipe_ingredients
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
