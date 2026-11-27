import type {
	CreateRecipeRequest,
	UpdateRecipeRequest,
	RecipeResponse,
	RecipeIngredientResponse,
	CreateRecipeIngredientRequest
} from '$bindings';
import { Create, Delete, GetAll, GetById, methods, Update } from './client';

const recipesPath = '/recipes';
const ingredientsPath = '/ingredients';

function recipeIngredientsPath(id: string) {
	return `${recipesPath}/${id}${ingredientsPath}`;
}

export const recipes = {
	...GetAll<RecipeResponse>(recipesPath),
	...GetById<RecipeResponse>(recipesPath),
	...Create<RecipeResponse, CreateRecipeRequest>(recipesPath),
	...Update<RecipeResponse, UpdateRecipeRequest>(recipesPath),
	...Delete(recipesPath),
	getIngredientsById: (id: string): Promise<RecipeIngredientResponse> => {
		return methods.get<RecipeIngredientResponse>(recipeIngredientsPath(id));
	},
	createIngredients: (id: string, ingredients: CreateRecipeIngredientRequest[]) => {
		return methods.post<RecipeIngredientResponse[]>(recipeIngredientsPath(id), ingredients);
	}
};
