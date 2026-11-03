import type { PatchRecipeSchema, PostRecipeSchema, RecipeSchema } from '$bindings';
import { api } from './request';

const recipesPath = '/recipes';
const recipePath = (id: string) => `${recipesPath}/${id}`;

export const recipes = {
	async all() {
		return await api.get<RecipeSchema>(recipesPath);
	},

	async by_id(recipe_id: string) {
		return await api.get<RecipeSchema>(recipePath(recipe_id));
	},

	async create(recipe: PostRecipeSchema) {
		return await api.post<RecipeSchema>(recipesPath, recipe);
	},

	async update(recipe_id: string, recipe: PatchRecipeSchema) {
		return await api.patch<RecipeSchema>(recipePath(recipe_id), recipe);
	},

	async delete(recipe_id: string) {
		return await api.delete(recipe_id);
	}
};
