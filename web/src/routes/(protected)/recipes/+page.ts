import { api } from '#lib/api/instance.ts';

export async function load() {
	try {
		const recipes = await api.recipes.getAll();
		return { recipes };
	} catch (error) {
		console.log(JSON.stringify(error, null, 2));
	}
}
