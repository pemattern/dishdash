import type { PatchRecipeSchema, PostRecipeSchema, RecipeSchema } from '$bindings';
import { Create, Delete, GetAll, GetById, Update } from './client';

const recipesPath = '/recipes' as const;

export const recipes = {
	...GetAll<RecipeSchema>(recipesPath),
	...GetById<RecipeSchema>(recipesPath),
	...Create<RecipeSchema, PostRecipeSchema>(recipesPath),
	...Update<RecipeSchema, PatchRecipeSchema>(recipesPath),
	...Delete(recipesPath)
};
