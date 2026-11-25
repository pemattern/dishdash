use lib::BINDINGS_PATH;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "lowercase")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "locale_code", rename_all = "lowercase")]
pub enum LocaleCode {
    De,
    En,
    Es,
    Fr,
    It,
    #[serde(rename = "pt-BR")]
    #[sqlx(rename = "pt-BR")]
    PtBr,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "meal_type_code", rename_all = "kebab-case")]
pub enum MealTypeCode {
    Breakfast,
    Brunch,
    Lunch,
    Dinner,
    Snack,
    Dessert,
    Appetizer,
    SideDish,
    Drink,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "physical_quantity_code", rename_all = "kebab-case")]
pub enum PhysicalQuantityCode {
    Count,
    Mass,
    Volume,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "unit_system_code", rename_all = "kebab-case")]
pub enum UnitSystemCode {
    Imperial,
    Metric,
    UsCustomary,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "unit_code", rename_all = "kebab-case")]
pub enum UnitCode {
    Imperial,
    Metric,
    UsCustomary,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "ingredient_category_code", rename_all = "kebab-case")]
pub enum IngredientCategoryCode {
    BakingIngredients,
    CannedPreserved,
    DairyEggs,
    FlavorEnhancer,
    Fruits,
    GrainsCereal,
    HerbsSpices,
    Legumes,
    Liquids,
    Meat,
    NutsSeeds,
    OilsFats,
    PlantBasedProteins,
    SaucesCondiments,
    Seafood,
    Sweeteners,
    Vegetables,
}
