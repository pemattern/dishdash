<script lang="ts">
	import { routes } from '#lib/const/routes.ts';
	import { resolve } from '$app/paths';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();
</script>

<h1>Recipes</h1>

{#each data.recipes as recipe (recipe.id)}
	<article>
		<h2>
			<a href={resolve(routes.recipe, { id: recipe.id })}>
				{recipe.name}
			</a>
		</h2>

		<p>{recipe.summary}</p>
	</article>
{/each}
