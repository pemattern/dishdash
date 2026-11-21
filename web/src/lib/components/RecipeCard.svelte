<script lang="ts">
	import type { RecipeSchema } from '$bindings';
	type Props = {
		recipe: RecipeSchema;
	};
	let { recipe }: Props = $props();
</script>

<article
	class="overflow-hidden rounded-xl bg-off-white shadow-sm inset-ring inset-ring-light-green transition-shadow hover:bg-white hover:shadow-md"
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
