use sqlx::PgExecutor;

use crate::{
    error::AppResult,
    models::{ingredient::Ingredient, search::ingredient::IngredientSearch},
    requests::search_query::SearchQuery,
};

pub struct Ingredients;

impl Ingredients {
    pub async fn select_all<'a>(executor: impl PgExecutor<'a>) -> AppResult<Vec<Ingredient>> {
        let ingredients = sqlx::query_as!(Ingredient, "SELECT * FROM ingredients;")
            .fetch_all(executor)
            .await?;
        Ok(ingredients)
    }

    pub async fn search<'a>(
        executor: impl PgExecutor<'a>,
        search_query: SearchQuery,
    ) -> AppResult<Vec<IngredientSearch>> {
        let ingredients = sqlx::query_as!(
            IngredientSearch,
            r#"WITH search_input AS (
                SELECT search_normalize($1) AS query
            ),
            matches AS (
                SELECT
                    i.code,
                    i.ingredient_category_code,
                    t.name,
                    t.name_plural,
                    t.name_normalized,
                    CASE
                        WHEN t.name_normalized = s.query THEN 1
                        WHEN t.name_normalized LIKE s.query || '%' THEN 2
                        ELSE 4
                    END AS match_priority,
                    similarity(t.name_normalized, s.query) AS similarity
                FROM ingredients i
                JOIN ingredient_translations t
                    ON t.ingredient_code = i.code
                CROSS JOIN search_input s
                WHERE t.locale_code = $2
                  AND (
                      t.name_normalized = s.query
                      OR t.name_normalized LIKE s.query || '%'
                      OR t.name_normalized % s.query
                  )
                UNION ALL
                SELECT
                    i.code,
                    i.ingredient_category_code,
                    t.name,
                    t.name_plural,
                    a.name_normalized,
                    CASE
                        WHEN a.name_normalized = s.query THEN 1
                        WHEN a.name_normalized LIKE s.query || '%' THEN 3
                        ELSE 5
                    END AS match_priority,
                    similarity(a.name_normalized, s.query) AS similarity
                FROM ingredient_aliases a
                JOIN ingredients i
                    ON i.code = a.ingredient_code
                JOIN ingredient_translations t
                    ON t.ingredient_code = a.ingredient_code
                   AND t.locale_code = a.locale_code
                CROSS JOIN search_input s
                WHERE a.locale_code = $2
                  AND (
                      a.name_normalized = s.query
                      OR a.name_normalized LIKE s.query || '%'
                      OR a.name_normalized % s.query
                  )
            )
            SELECT
                code AS "code!",
                ingredient_category_code AS "ingredient_category_code!",
                name AS "name!"
            FROM matches
            GROUP BY
                code,
                ingredient_category_code,
                name
            ORDER BY
                MIN(match_priority),
                MAX(similarity) DESC,
                name
            LIMIT $3;"#,
            search_query.q,
            search_query.locale,
            search_query.limit.unwrap_or(10),
        )
        .fetch_all(executor)
        .await?;
        Ok(ingredients)
    }
}
