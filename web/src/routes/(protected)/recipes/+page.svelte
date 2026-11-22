<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import FormModal from '#lib/components/FormModal.svelte';
	import Input from '#lib/components/Input.svelte';
	import RecipeCard from '#lib/components/RecipeCard.svelte';
	import { refreshAll } from '$app/navigation';
	import type { CreateRecipeRequest, MealType } from '$bindings';
	import type { PageProps } from './$types';
	import { m } from '#lib/paraglide/messages.js';
	import MealTypeSelector from '#lib/components/MealTypeSelector.svelte';

	let { data }: PageProps = $props();

	let open = $state(false);
	let name: string = $state('');
	let imageData: string | null = $state(null);
	let durationMinutes: number = $state(0);
	let mealTypes: MealType[] = $state([]);
	let error = $state('');

	$effect(() => {
		if (!open) {
			resetForm();
		}
	});

	function resetForm() {
		name = '';
		error = '';
	}

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		error = '';

		const request: CreateRecipeRequest = {
			name,
			imageData,
			durationMinutes,
			mealTypes
		};

		try {
			await api.recipes.create(request);
			await refreshAll();
			open = false;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		}
	}
</script>

<button
	class="m-4 rounded-full bg-orange px-3 py-2 font-sans text-sm font-bold text-off-white inset-ring inset-ring-orange hover:cursor-pointer hover:bg-off-white hover:text-orange hover:inset-ring-orange"
	onclick={() => (open = true)}
>
	{m.newRecipe()}</button
>

<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
	{#each data.recipes as recipe (recipe.id)}
		<RecipeCard {recipe} />
	{/each}
</div>

<FormModal bind:open {handleSubmit}>
	<Input
		bind:value={name}
		name="name"
		type="text"
		focus={open}
		label={m.title()}
		placeholder="Oven veggies"
	/>
	<MealTypeSelector bind:selected={mealTypes} />
	{error}
</FormModal>
