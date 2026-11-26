<script lang="ts">
	import RecipeCard from '#lib/components/RecipeCard.svelte';
	import type { PageProps } from './$types';
	import { m } from '#lib/paraglide/messages.js';
	import { routes } from '#lib/const/routes.ts';
	import { resolve } from '$app/paths';

	let { data }: PageProps = $props();
</script>

<a
	href={resolve(routes.recipeNew)}
	class="m-4 rounded-full bg-orange px-3 py-2 font-sans text-sm font-semibold text-off-white inset-ring inset-ring-orange hover:cursor-pointer hover:bg-off-white hover:text-orange hover:inset-ring-orange"
>
	+ {m.newRecipe()}
</a>

<div class="mx-auto grid w-fit grid-cols-1 justify-center gap-6 pt-4 sm:grid-cols-2 lg:grid-cols-3">
	{#each data.recipes as recipe (recipe.id)}
		<div class="w-full max-w-100 min-w-0"><RecipeCard {recipe} /></div>
	{/each}
</div>
