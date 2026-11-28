<script lang="ts">
	import { m } from '#lib/paraglide/messages.js';
	import { appState } from '#lib/state.svelte.ts';
	import type { RecipeIngredient, Unit } from '#lib/types/unit.ts';
	import Input from './Input.svelte';
	import Label from './Label.svelte';
	import SearchableSelect from './SearchableSelect.svelte';

	type Props = {
		value: RecipeIngredient[];
	};

	let { value = $bindable() }: Props = $props();

	let name = $state('');
	let quantity = $state('');
	let unit = $state<Unit | null>(null);
	let ingredient = $state<string>('');

	function addIngredient() {
		if (!name.trim()) {
			return;
		}

		if (quantity && Number(quantity) <= 0) {
			return;
		}

		value.push({
			ingredientCode: ingredient,
			unitCode: 'piece',
			unitSystemCode: 'imperial',
			quantity: parseFloat(quantity),
			position: 0
		});

		quantity = '';
		name = '';
	}

	function removeIngredient(ingredient_code: string) {
		value = value.filter((item) => item.ingredientCode !== ingredient_code);
	}
</script>

<div>
	<Label for="addIngredientButton" required={true}>{m.selectIngredients()}</Label>
	<div>
		{#each value as ingredient (ingredient)}
			<div class="flex min-h-13 items-center gap-3 border-orange/15">
				<div class="min-w-0 flex-1 text-sm text-orange">
					<span>
						{ingredient.quantity}
						{ingredient.unitCode}
					</span>
					<span>
						{ingredient.ingredientCode}
					</span>
				</div>

				<button
					type="button"
					onclick={() => removeIngredient(ingredient.ingredientCode)}
					aria-label="Remove ingredient"
					class="rounded-full px-2 py-1 text-lg text-orange/50 transition hover:bg-orange/10 hover:text-orange"
				>
					×
				</button>
			</div>
		{/each}
	</div>
	<div class="flex flex-row items-center gap-2">
		<div class="w-24">
			<Input bind:value={quantity} type="number" min="0" step="any" placeholder="Qty" />
		</div>
		<div class="w-24">
			<SearchableSelect
				bind:value={unit}
				options={appState.units}
				getLabel={(unit) => unit?.code.toString() ?? ''}
			/>
		</div>
		<div class="flex-8">
			<SearchableSelect
				bind:value={ingredient}
				placeholder="Ingredient"
				getLabel={(ingredient) => ingredient}
			/>
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
