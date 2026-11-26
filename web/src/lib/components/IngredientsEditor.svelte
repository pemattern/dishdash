<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import Input from './Input.svelte';
	import Label from './Label.svelte';
	import SearchableSelect from './SearchableSelect.svelte';

	type Ingredient = {
		id: string;
		quantity: string;
		unit: string;
		name: string;
	};

	let name = $state('');
	let ingredients = $state<Ingredient[]>([]);
	let quantity = $state('');
	let unit = $state('g');

	const units = ['g', 'kg', 'ml', 'l', 'tsp', 'tbsp', 'cup', 'piece', 'clove', 'pinch'];

	function addIngredient() {
		if (!name.trim()) {
			return;
		}

		if (quantity && Number(quantity) <= 0) {
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
</script>

<div>
	<Label for="addIngredientButton" required={true}>{m.selectIngredients()}</Label>

	<!-- Existing ingredients -->
	<div>
		{#each ingredients as ingredient (ingredient.id)}
			<div class="flex min-h-13 items-center gap-3 border-orange/15">
				<div class="min-w-0 flex-1 text-sm text-orange">
					<span>
						{#if ingredient.quantity}
							{ingredient.quantity} {ingredient.unit}
						{:else}
							To taste
						{/if}
					</span>
					<span>
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
	<div class="flex flex-row items-center gap-2">
		<div class="flex-1">
			<Input bind:value={quantity} type="number" min="0" step="any" placeholder="Qty" />
		</div>
		<div class="flex-1">
			<SearchableSelect bind:value={unit} options={units} />
		</div>
		<div class="flex-8">
			<Input bind:value={name} placeholder="Ingredient" aria-label="Ingredient name" />
		</div>

		<button
			onclick={(event) => {
				event.preventDefault();
				addIngredient();
			}}
			type="submit"
			id="addIngredientButton"
			disabled={!name.trim()}
			aria-label="Add ingredient"
			class="flex size-10 shrink-0 items-center justify-center rounded-full bg-orange text-xl font-semibold text-off-white transition hover:bg-off-white hover:text-orange hover:inset-ring hover:inset-ring-orange disabled:cursor-not-allowed disabled:opacity-40"
		>
			+
		</button>
	</div>
</div>
