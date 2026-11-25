CREATE TYPE ingredient_category_code AS ENUM (
    'baking-ingredients',
    'canned-preserved',
    'dairy-eggs',
    'flavor-enhancer',
    'fruits',
    'grains-cereal',
    'herbs-spices',
    'legumes',
    'liquids',
    'meat',
    'nuts-seeds',
    'oils-fats',
    'plant-based-proteins',
    'sauces-condiments',
    'seafood',
    'sweeteners',
    'vegetables'
);

CREATE TABLE ingredients (
    code TEXT PRIMARY KEY,
    ingredient_category_code ingredient_category_code NOT NULL
);

CREATE TABLE ingredient_translations (
    ingredient_code TEXT NOT NULL REFERENCES ingredients(code),
    locale_code locale_code NOT NULL,
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
    locale_code locale_code NOT NULL,
    name TEXT NOT NULL,
    name_plural TEXT NOT NULL,
    name_normalized TEXT NOT NULL,
    PRIMARY KEY (ingredient_code, locale_code, name),
    FOREIGN KEY (ingredient_code, locale_code)
        REFERENCES ingredient_translations (ingredient_code, locale_code)
        ON DELETE CASCADE
);

CREATE INDEX ingredient_aliases_normalized_index
ON ingredient_aliases (
    locale_code,
    name_normalized
);

CREATE INDEX ingredient_aliases_prefix_index
ON ingredient_aliases (locale_code, name_normalized text_pattern_ops);

CREATE INDEX ingredient_aliases_trgm_index
ON ingredient_aliases
USING GIN (name_normalized gin_trgm_ops);
