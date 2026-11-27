<script lang="ts">
	import type { ingredients } from '#lib/api/ingredients.ts';
	import { api } from '#lib/api/instance.ts';
	import ImageUpload from '#lib/components/ImageUpload.svelte';
	import IngredientsEditor from '#lib/components/IngredientsEditor.svelte';
	import Input from '#lib/components/Input.svelte';
	import MealTypeSelector from '#lib/components/MealTypeSelector.svelte';
	import { routes } from '#lib/const/routes.ts';
	import { m } from '#lib/paraglide/messages.js';
	import { goto } from '$app/navigation';
	import { resolve } from '$app/paths';
	import type { CreateRecipeRequest, MealTypeCode, RecipeIngredientResponse, RecipeResponse } from '$bindings';
	import { untrack } from 'svelte';

	type Props = {
		recipe?: RecipeResponse;
		ingredients?: RecipeIngredientResponse[];
	};

	let { recipe, ingredients }: Props = $props();

	let name: string = $state(untrack(() => recipe?.name) ?? '');
	let imageData: string | null = $state(null);
	let durationMinutes: number = $state(untrack(() => recipe?.durationMinutes) ?? 0);
	let durationMinutesString: string = $state(
		untrack(() => recipe?.durationMinutes.toString()) ?? ''
	);
	let mealTypes: MealTypeCode[] = $state(untrack(() => recipe?.mealTypes) ?? []);

	$effect(() => {
		let value = parseInt(durationMinutesString);
		if (!isNaN(value)) {
			durationMinutes = value;
		}
	});

	async function handleSubmit(event: SubmitEvent) {
		event.preventDefault();
		if (recipe) {
			await handleCreate();
		} else {
			await handleUpdate();
		}
	}

	async function handleCreate() {
		const request: CreateRecipeRequest = {
			name,
			imageData,
			durationMinutes,
			mealTypes
		};
		try {
			const recipeResponse = await api.recipes.create(request);
			const recipeIngredientResponse = await api.
			await goto(resolve(routes.recipe, { id: recipeResponse.id }));
		} catch (err) {
			console.log(err);
		}
	}

	async function handleUpdate() {}

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

	async function handleUpload(file: File) {
		try {
			imageData = await fileToBase64(file);
		} catch (error) {
			console.error('Image conversion failed:', error);
		}
	}
</script>

<div class="mx-auto w-full max-w-5xl px-4 py-6 font-sans sm:px-6">
	<form class="flex flex-col gap-8" onsubmit={handleSubmit}>
		<div class="flex flex-row gap-8">
			<div class="flex min-w-0 flex-1 flex-col gap-8">
				<Input
					bind:value={name}
					name="name"
					type="text"
					required={true}
					autofocus
					label={m.title()}
					placeholder="Oven veggies"
				/>
				<Input
					bind:value={durationMinutesString}
					name="durationMinutesInput"
					type="number"
					required={true}
					label={m.preparationTime()}
					step="5"
					placeholder="25"
				/>
				<MealTypeSelector required={true} label={m.selectMealTypes()} bind:selected={mealTypes} />
			</div>
			<div class="min-w-0 flex-1">
				<ImageUpload label={m.uploadCoverPhoto()} onUpload={handleUpload} url={recipe?.imageUrl} />
			</div>
		</div>
		<IngredientsEditor bind:value={ingredients} />
		<button class="rounded-xl bg-off-white p-4 font-medium text-orange">{m.save()}</button>
	</form>
</div>
