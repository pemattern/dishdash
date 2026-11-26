import type {
	CreateRecipeRequest,
	UpdateRecipeRequest,
	RecipeResponse,
	RecipeIngredientResponse
} from '$bindings';
import { Create, Delete, GetAll, GetById, methods, Update } from './client';

const recipesPath = '/recipes';
const ingredientsPath = '/ingredients';

export const recipes = {
	...GetAll<RecipeResponse>(recipesPath),
	...GetById<RecipeResponse>(recipesPath),
	...Create<RecipeResponse, CreateRecipeRequest>(recipesPath),
	...Update<RecipeResponse, UpdateRecipeRequest>(recipesPath),
	...Delete(recipesPath),
	getIngredientsById: (id: string): Promise<RecipeIngredientResponse> => {
		return methods.get<RecipeIngredientResponse>(`${recipesPath}/${id}/${ingredientsPath}`);
	}
};
