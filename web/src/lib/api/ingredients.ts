import { type IngredientSchema } from '$bindings';
import { Delete, GetAll, GetById } from './client';

const ingredientsPath = '/ingredients';

export const ingredients = {
	...GetAll<IngredientSchema>(ingredientsPath),
	...GetById<IngredientSchema>(ingredientsPath),
	...Delete(ingredientsPath)
};
