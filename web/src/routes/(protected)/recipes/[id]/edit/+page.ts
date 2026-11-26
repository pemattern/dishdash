import { api } from '#lib/api/instance.ts';

export async function load({ params }) {
	try {
		const [recipe, ingredients] = await Promise.all([
			api.recipes.getById(params.id),
			api.recipes.getIngredientsById(params.id)
		]);
		return { recipe, ingredients };
	} catch (error) {
		console.log(JSON.stringify(error, null, 2));
	}
}
