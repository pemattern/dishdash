use lib::BINDINGS_PATH;
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "meal_type", rename_all = "kebab-case")]
pub enum MealType {
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
#[sqlx(type_name = "meal_type", rename_all = "kebab-case")]
pub enum PhysicalQuantity {
    Count,
    Mass,
    Volume,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "meal_type", rename_all = "kebab-case")]
pub enum UnitSystem {
    Imperial,
    Metric,
    UsCustomary,
}

#[derive(Serialize, Deserialize, ts_rs::TS, sqlx::Type)]
#[serde(rename_all = "kebab-case")]
#[ts(export, export_to = BINDINGS_PATH)]
#[sqlx(type_name = "meal_type", rename_all = "kebab-case")]
pub enum IngredientCategory {
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
    Vegetable,
}
