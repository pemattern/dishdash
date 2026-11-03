import {
	type IngredientSchema,
	type PatchIngredientSchema,
	type PostIngredientSchema
} from '$bindings';
import { Create, Delete, GetAll, GetById, Update } from './client';

const ingredientsPath = '/ingredients';

export const ingredients = {
	...GetAll<IngredientSchema>(ingredientsPath),
	...GetById<IngredientSchema>(ingredientsPath),
	...Create<IngredientSchema, PostIngredientSchema>(ingredientsPath),
	...Update<IngredientSchema, PatchIngredientSchema>(ingredientsPath),
	...Delete(ingredientsPath)
};
