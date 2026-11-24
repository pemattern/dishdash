<script lang="ts">
	import type { RecipeResponse } from '$bindings';
	type Props = {
		recipe: RecipeResponse;
	};
	let { recipe }: Props = $props();
</script>

<article
	class="overflow-hidden rounded-xl bg-white shadow-sm inset-ring inset-ring-light-orange transition-shadow hover:cursor-pointer hover:bg-off-white hover:shadow-md"
>
	<img src={recipe.imageUrl} alt={recipe.name} class="aspect-video w-full object-cover" />

	<div class="p-4">
		<h3 class="text-lg font-semibold text-dark-gray">
			{recipe.name}
		</h3>

		<div class="mt-2 flex items-center gap-2 text-sm text-gray">
			<span>⏱</span>
			<span>{recipe.durationMinutes} min</span>
		</div>
	</div>
</article>
