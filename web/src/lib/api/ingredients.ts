import { type IngredientResponse } from '$bindings';
import { Delete, GetAll, GetById } from './client';

const ingredientsPath = '/ingredients';

export const ingredients = {
	...GetAll<IngredientResponse>(ingredientsPath),
	...GetById<IngredientResponse>(ingredientsPath),
	...Delete(ingredientsPath)
};
