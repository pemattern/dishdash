<script lang="ts">
	import { mealTypeLabelMap } from '#lib/const/maps.ts';
	import type { RecipeResponse } from '$bindings';
	import { LucideSoup } from '@lucide/svelte';
	import Badge from './Badge.svelte';
	import { routes } from '#lib/const/routes.ts';
	import { resolve } from '$app/paths';
	import EditButton from './Buttons/EditButton.svelte';

	type Props = {
		recipe: RecipeResponse;
	};

	let { recipe }: Props = $props();

	const maxBadgeCount = 2;
</script>

<article
	class="group relative overflow-hidden rounded-xl bg-white shadow-sm inset-ring inset-ring-light-gray hover:bg-off-white"
>
	<a
		href={resolve(routes.recipe, { id: recipe.id })}
		class="absolute inset-0 z-0 rounded-xl focus-visible:outline-2 focus-visible:outline-offset-2 focus-visible:outline-lighter-green"
		aria-label="View {recipe.name}"
	></a>

	<div class="pointer-events-none relative">
		{#if recipe.imageUrl}
			<img src={recipe.imageUrl} alt="" class="aspect-video w-full object-cover" />
		{:else}
			<div
				class="flex aspect-video w-full items-center justify-center bg-lightest-green text-green"
			>
				<LucideSoup size="32" />
			</div>
		{/if}
		<a href={resolve(routes.recipeEdit, { id: recipe.id })} class="absolute top-3 right-3">
			<EditButton />
		</a>
	</div>

	<div class="pointer-events-none flex flex-row items-start gap-2 p-4">
		<div class="min-w-0 flex-1">
			<h3 class="font-serif text-xl font-semibold text-dark-gray">
				{recipe.name}
			</h3>

			<div class="mt-2 flex items-center gap-2 text-sm text-gray">
				<span class="text-lg">⏱</span>
				<span>{recipe.durationMinutes} min</span>
			</div>
		</div>

		<div class="ml-auto flex w-1/2 flex-wrap justify-end gap-2">
			{#each recipe.mealTypes.slice(0, maxBadgeCount) as option (option)}
				<Badge
					text={mealTypeLabelMap.get(option) ?? ''}
					bgColor="bg-lighter-green"
					textColor="text-white"
					borderColor="border-lighter-green"
				/>
			{/each}

			{#if recipe.mealTypes.length > maxBadgeCount}
				<p class="pt-1 text-sm font-bold text-lighter-green">
					+{recipe.mealTypes.length - maxBadgeCount}
				</p>
			{/if}
		</div>
	</div>
</article>
