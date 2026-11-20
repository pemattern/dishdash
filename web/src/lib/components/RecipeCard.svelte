<script lang="ts">
	import type { RecipeSchema } from '$bindings';
	type Props = {
		recipe: RecipeSchema;
	};
	let { recipe }: Props = $props();
</script>

<article
	class="border-gray-200 overflow-hidden rounded-xl border bg-white shadow-sm transition-shadow hover:shadow-md"
>
	<img src={recipe.imageUrl} alt={recipe.name} class="aspect-video w-full object-cover" />

	<div class="p-4">
		<h3 class="text-gray-900 text-lg font-semibold">
			{recipe.name}
		</h3>

		<div class="text-gray-500 mt-2 flex items-center gap-2 text-sm">
			<span>⏱</span>
			<span>{recipe.durationMinutes} min</span>
		</div>
	</div>
</article>
