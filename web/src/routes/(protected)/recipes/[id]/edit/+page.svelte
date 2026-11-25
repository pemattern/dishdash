<script lang="ts">
	import { api } from '#lib/api/instance.ts';
	import type { PageProps } from './$types';

	type Ingredient = {
		id: string;
		quantity: string;
		unit: string;
		name: string;
	};

	let { data }: PageProps = $props();

	let ingredients = $state<Ingredient[]>([]);
	let quantity = $state('');
	let unit = $state('g');
	let name = $state('');
	let error = $state('');
	let saving = $state(false);
	let success = $state(false);

	const units = ['g', 'kg', 'ml', 'l', 'tsp', 'tbsp', 'cup', 'piece', 'clove', 'pinch'];

	function addIngredient() {
		error = '';
		success = false;

		if (!name.trim()) {
			error = 'Enter an ingredient name.';
			return;
		}

		if (quantity && Number(quantity) <= 0) {
			error = 'Quantity must be greater than zero.';
			return;
		}

		ingredients.push({
			id: crypto.randomUUID(),
			quantity,
			unit,
			name: name.trim()
		});

		quantity = '';
		unit = 'g';
		name = '';
	}

	function removeIngredient(id: string) {
		ingredients = ingredients.filter((item) => item.id !== id);
	}

	async function saveIngredients() {
		if (!ingredients.length || saving) return;

		error = '';
		success = false;
		saving = true;

		try {
			// Replace with your actual ingredient API method.
			// await api.recipes.updateIngredients(data.recipe.id, {
			// 	ingredients: ingredients.map(({ id, ...item }) => ({
			// 		...item,
			// 		quantity: item.quantity ? Number(item.quantity) : null
			// 	}))
			// });

			success = true;
		} catch (err) {
			error = err instanceof Error ? err.message : 'Something went wrong';
		} finally {
			saving = false;
		}
	}
</script>

<div class="mx-auto w-full max-w-3xl px-4 py-6 font-sans sm:px-6">
	<!-- Header -->
	<div class="mb-8">
		<p class="p-4 font-serif text-4xl font-medium text-green italic sm:text-3xl">
			{data.recipe.name}
		</p>
		<img src={data.recipe?.imageUrl} alt={data.recipe.name} />
	</div>

	<!-- Ingredient editor -->
	<div class="mb-5 flex items-center justify-between">
		<h2 class="text-lg font-bold text-orange">Ingredients</h2>

		<span class="text-sm text-orange">
			{ingredients.length} added
		</span>
	</div>

	<!-- Existing ingredients -->
	<div>
		{#each ingredients as ingredient, i (ingredient.id)}
			<div class="flex min-h-13 items-center gap-3 border-b border-orange/15 py-3">
				<span class="w-6 shrink-0 text-xs font-semibold text-orange/60">
					{String(i + 1).padStart(2, '0')}
				</span>

				<div class="min-w-0 flex-1 text-sm text-orange">
					<span class="font-bold">
						{#if ingredient.quantity}
							{ingredient.quantity} {ingredient.unit}
						{:else}
							To taste
						{/if}
					</span>

					<span class="ml-1">
						{ingredient.name}
					</span>
				</div>

				<button
					type="button"
					onclick={() => removeIngredient(ingredient.id)}
					aria-label="Remove {ingredient.name}"
					class="rounded-full px-2 py-1 text-lg text-orange/50 transition hover:bg-orange/10 hover:text-orange"
				>
					×
				</button>
			</div>
		{/each}
	</div>

	<!-- New ingredient row -->
	<form
		onsubmit={(event) => {
			event.preventDefault();
			addIngredient();
		}}
		class="flex items-center gap-2 pt-4"
	>
		<span class="w-6 shrink-0 text-xs font-semibold text-orange/60">
			{String(ingredients.length + 1).padStart(2, '0')}
		</span>

		<input
			bind:value={quantity}
			type="number"
			min="0"
			step="any"
			placeholder="Qty"
			aria-label="Quantity"
			class="w-16 min-w-0 rounded-lg border border-orange/30 bg-white px-2 py-2.5 text-sm text-orange outline-none placeholder:text-orange/40 focus:border-orange sm:w-20"
		/>

		<select
			bind:value={unit}
			aria-label="Unit"
			class="w-20 shrink-0 rounded-lg border border-orange/30 bg-white px-2 py-2.5 text-sm text-orange outline-none focus:border-orange sm:w-24"
		>
			{#each units as item}
				<option value={item}>{item}</option>
			{/each}
		</select>

		<input
			bind:value={name}
			placeholder="Ingredient"
			aria-label="Ingredient name"
			class="min-w-0 flex-1 rounded-lg border border-orange/30 bg-white px-3 py-2.5 text-sm text-orange outline-none placeholder:text-orange/40 focus:border-orange"
		/>

		<button
			type="submit"
			disabled={!name.trim()}
			aria-label="Add ingredient"
			class="flex size-10 shrink-0 items-center justify-center rounded-full bg-orange text-xl font-semibold text-off-white transition hover:bg-off-white hover:text-orange hover:inset-ring hover:inset-ring-orange disabled:cursor-not-allowed disabled:opacity-40"
		>
			+
		</button>
	</form>

	{#if error}
		<p class="text-red-600 mt-3 text-sm">{error}</p>
	{/if}

	{#if success}
		<p class="text-green-700 mt-3 text-sm">Ingredients saved successfully!</p>
	{/if}

	<!-- Footer -->
	<div class="mt-6 flex justify-end border-t border-orange/15 pt-5">
		<button
			type="button"
			onclick={saveIngredients}
			disabled={!ingredients.length || saving}
			class="rounded-full bg-orange px-5 py-2.5 text-sm font-semibold text-off-white inset-ring inset-ring-orange transition hover:bg-off-white hover:text-orange disabled:cursor-not-allowed disabled:opacity-40"
		>
			{saving ? 'Saving...' : 'Save ingredients'}
		</button>
	</div>
</div>
