<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import FormModal from '#lib/components/FormModal.svelte';
	import Input from '#lib/components/Input.svelte';
	import RecipeCard from '#lib/components/RecipeCard.svelte';
	import { refreshAll } from '$app/navigation';
	import type { CreateRecipeRequest, MealTypeCode } from '$bindings';
	import type { PageProps } from './$types';
	import { m } from '#lib/paraglide/messages.js';
	import MealTypeSelector from '#lib/components/MealTypeSelector.svelte';
	import ImageUpload from '#lib/components/ImageUpload.svelte';

	let { data }: PageProps = $props();

	let open = $state(false);
	let name: string = $state('');
	let imageData: string | null = $state(null);
	let durationMinutes: number = $state(0);
	let durationMinutesString: string = $state('');
	let mealTypes: MealTypeCode[] = $state([]);
	let error = $state('');

	$effect(() => {
		if (!open) {
			resetForm();
		}
	});

	$effect(() => {
		let value = parseInt(durationMinutesString);
		if (!isNaN(value)) {
			durationMinutes = value;
		}
	});

	function resetForm() {
		name = '';
		mealTypes = [];
		durationMinutes = 0;
		imageData = null;
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

	async function fileToBase64(file: File): Promise<string> {
		return new Promise((resolve, reject) => {
			const reader = new FileReader();
			reader.onload = () => {
				const result = reader.result as string;
				resolve(result);
			};
			reader.onerror = () => reject(reader.error);
			reader.readAsDataURL(file);
		});
	}

	async function handleUpload(files: File[]) {
		try {
			const base64Images = await Promise.all(files.map(async (file) => await fileToBase64(file)));
			imageData = base64Images[0];
		} catch (error) {
			console.error('Image conversion failed:', error);
		}
	}
</script>

<button
	class="m-4 rounded-full bg-orange px-3 py-2 font-sans text-sm font-semibold text-off-white inset-ring inset-ring-orange hover:cursor-pointer hover:bg-off-white hover:text-orange hover:inset-ring-orange"
	onclick={() => (open = true)}
>
	+ {m.newRecipe()}</button
>

<div class="mx-auto grid w-fit grid-cols-1 justify-center gap-6 sm:grid-cols-2 lg:grid-cols-3">
	{#each data.recipes as recipe (recipe.id)}
		<div class="w-full max-w-100 min-w-0"><RecipeCard {recipe} /></div>
	{/each}
</div>

<FormModal bind:open {handleSubmit}>
	<div class="flex flex-row gap-4">
		<Input
			bind:value={name}
			name="name"
			type="text"
			focus={open}
			label={m.title()}
			placeholder="Oven veggies"
		/>
		<Input
			bind:value={durationMinutesString}
			name="durationMinutesInput"
			type="number"
			label={m.preparationTime()}
			placeholder="25"
		/>
	</div>
	<MealTypeSelector label={m.selectMealTypes()} bind:selected={mealTypes} />
	<ImageUpload label={m.uploadCoverPhoto()} onUpload={handleUpload} />
	{error}
</FormModal>
