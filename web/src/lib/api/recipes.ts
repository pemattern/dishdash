import type { CreateRecipeRequest, UpdateRecipeRequest, RecipeResponse } from '$bindings';
import { Create, Delete, GetAll, GetById, Update } from './client';

const recipesPath = '/recipes' as const;

export const recipes = {
	...GetAll<RecipeResponse>(recipesPath),
	...GetById<RecipeResponse>(recipesPath),
	...Create<RecipeResponse, CreateRecipeRequest>(recipesPath),
	...Update<RecipeResponse, UpdateRecipeRequest>(recipesPath),
	...Delete(recipesPath)
};
