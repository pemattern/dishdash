<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import FormModal from '#lib/components/FormModal.svelte';
	import Input from '#lib/components/Input.svelte';
	import RecipeCard from '#lib/components/RecipeCard.svelte';
	import { refreshAll } from '$app/navigation';
	import type { PostRecipeSchema } from '$bindings';
	import type { PageProps } from './$types';

	let { data }: PageProps = $props();

	let open = $state(false);
	let name = $state('');
	let imageData = $state('');
	let durationMinutes = $state(1);
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

		const schema: PostRecipeSchema = {
			name,
			imageData,
			durationMinutes
		};

		try {
			await api.recipes.create(schema);
			await refreshAll();
			open = false;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		}
	}
</script>

<button class="m-4 rounded-md bg-lighter-green p-4 font-sans" onclick={() => (open = true)}
	>Create</button
>

<div class="grid grid-cols-1 gap-6 sm:grid-cols-2 lg:grid-cols-3">
	{#each data.recipes as recipe (recipe.id)}
		<RecipeCard {recipe} />
	{/each}
</div>

<FormModal bind:open {handleSubmit}>
	<Input label="Title" bind:value={name} name="name" type="text" focus={open} />
	{error}
</FormModal>
