CREATE TABLE recipe_ingredients (
    recipe_id     UUID NOT NULL REFERENCES recipes(id),
    ingredient_id UUID NOT NULL REFERENCES ingredients(id),
    quantity      NUMERIC(10, 2),
    unit          TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
    PRIMARY KEY (recipe_id, ingredient_id)
);

CREATE TRIGGER update_recipe_ingredients_updated_at
    BEFORE UPDATE ON recipe_ingredients
    FOR EACH ROW
    EXECUTE FUNCTION update_updated_at_column();
