<script lang="ts">
	import type { MealType } from '$bindings';

	let {
		value = $bindable<MealType[]>([])
	}: {
		value: MealType[];
	} = $props();

	function toggle(type: MealType) {
		value = value.includes(type) ? value.filter((item) => item !== type) : [...value, type];
	}

	const labels: Record<MealType, string> = {
		breakfast: 'Breakfast',
		lunch: 'Lunch',
		dinner: 'Dinner',
		snack: 'Snack',
		dessert: 'Dessert',
		brunch: 'Brunch'
	};
</script>

<div class="flex flex-wrap gap-2">
	{#each MEAL_TYPES as type}
		<button
			type="button"
			onclick={() => toggle(type)}
			aria-pressed={value.includes(type)}
			class="rounded-full border px-4 py-2 text-sm transition-colors"
			class:border-orange-500={value.includes(type)}
			class:bg-orange-500={value.includes(type)}
			class:text-white={value.includes(type)}
			class:border-gray-300={!value.includes(type)}
		>
			{labels[type]}
		</button>
	{/each}
</div>
