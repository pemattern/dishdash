import { api } from '#lib/api/instance.ts';

export async function load({ params }) {
	try {
		const recipe = await api.recipes.getById(params.id);
		return { recipe };
	} catch (error) {
		console.log(JSON.stringify(error, null, 2));
	}
}
