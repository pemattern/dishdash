CREATE TABLE ingredient_categories (
    code TEXT PRIMARY KEY
);

INSERT INTO ingredient_categories (code)
VALUES
    ('vegetables'),
    ('fruits'),
    ('dairy'),
    ('meat'),
    ('seafood'),
    ('grains_cereal'),
    ('legumes'),
    ('nuts_seeds'),
    ('cooking_liquids'),
    ('herbs_spices'),
    ('sweeteners'),
    ('sauces_condiments'),
    ('plant_based_proteins'),
    ('baking'),
    ('canned_preserved'),
    ('beverages');

CREATE TABLE ingredients (
    code TEXT PRIMARY KEY,
    ingredient_category_code TEXT NOT NULL REFERENCES ingredient_categories(code) ON DELETE RESTRICT
);

CREATE TABLE locales (
    code TEXT PRIMARY KEY
);

INSERT INTO locales (code)
VALUES
    ('en'),
    ('de');



CREATE TABLE ingredient_names (
    ingredient_code TEXT NOT NULL REFERENCES ingredients(code),
    locale_code TEXT NOT NULL REFERENCES locales(code),
    name TEXT NOT NULL,
    name_normalized TEXT NOT NULL,
    name_plural TEXT NOT NULL,
    PRIMARY KEY (ingredient_code, locale_code, name),
    UNIQUE (ingredient_code, locale_code, name_normalized)
);

CREATE INDEX ingredient_names_prefix_idx
ON ingredient_names (
    locale_code,
    name_normalized text_pattern_ops
);

CREATE INDEX ingredient_names_trgm_idx
ON ingredient_names
USING GIN (name_normalized gin_trgm_ops);
