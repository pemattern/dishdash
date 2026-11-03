import { routes } from '#lib/const/routes.ts';
import type { RecipeSchema } from '$bindings';
import { api } from './request';

export const recipes = {
	async get_all() {
		return await api.get<RecipeSchema[]>(routes.recipes);
	}
};
