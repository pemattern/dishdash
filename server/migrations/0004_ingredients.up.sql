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

CREATE TABLE ingredient_translations (
    ingredient_code TEXT NOT NULL REFERENCES ingredients(code),
    locale_code TEXT NOT NULL REFERENCES locales(code),
    name TEXT NOT NULL,
    name_plural TEXT NOT NULL,
    name_normalized TEXT NOT NULL,
    PRIMARY KEY (ingredient_code, locale_code)
);

CREATE INDEX ingredient_translations_normalized_index
ON ingredient_translations (
    locale_code,
    name_normalized
);

CREATE INDEX ingredient_translations_prefix_index
ON ingredient_translations (locale_code, name_normalized text_pattern_ops);

CREATE INDEX ingredient_translations_trgm_index
ON ingredient_translations
USING GIN (name_normalized gin_trgm_ops);

CREATE TABLE ingredient_aliases (
    ingredient_code TEXT NOT NULL REFERENCES ingredients(code),
    locale_code TEXT NOT NULL REFERENCES locales(code),
    name TEXT NOT NULL,
    name_plural TEXT NOT NULL,
    name_normalized TEXT NOT NULL,
    PRIMARY KEY (ingredient_code, locale_code, name),
    FOREIGN KEY (ingredient_code, locale_code)
        REFERENCES ingredient_translations (ingredient_code, locale_code)
        ON DELETE CASCADE
);

CREATE INDEX ingredient_aliases_normalized_index
ON ingredient_translations (
    locale_code,
    name_normalized
);

CREATE INDEX ingredient_aliases_prefix_index
ON ingredient_aliases (locale_code, name_normalized text_pattern_ops);

CREATE INDEX ingredient_aliases_trgm_index
ON ingredient_translations
USING GIN (name_normalized gin_trgm_ops);
