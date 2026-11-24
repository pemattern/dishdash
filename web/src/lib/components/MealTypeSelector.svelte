<script lang="ts">
	import * as m from '#lib/paraglide/messages.js';
	import type { MealTypeCode } from '$bindings';
	import Badge from './Badge.svelte';

	type Props = {
		selected?: MealTypeCode[];
	};

	let { selected = $bindable<MealTypeCode[]>([]) }: Props = $props();

	const options = new Map<MealTypeCode, string>([
		['breakfast', m.breakfast()],
		['brunch', m.brunch()],
		['lunch', m.lunch()],
		['dinner', m.dinner()],
		['snack', m.snack()],
		['dessert', m.dessert()],
		['appetizer', m.appetizer()],
		['side-dish', m.sideDish()],
		['drink', m.drink()]
	]);

	function toggle(mealTypeCode: MealTypeCode) {
		if (selected.includes(mealTypeCode)) {
			selected = selected.filter((item) => item !== mealTypeCode);
		} else {
			selected.push(mealTypeCode);
		}
	}

	function isSelected(mealTypeCode: MealTypeCode) {
		return selected.includes(mealTypeCode);
	}
</script>

<div class="flex flex-wrap gap-2" role="group">
	{#each options as option (option)}
		<button
			class="rounded-full hover:cursor-pointer"
			type="button"
			onclick={() => toggle(option[0])}
		>
			<Badge
				text={option[1]}
				bgColor={isSelected(option[0])
					? 'bg-lighter-green hover:bg-light-green'
					: 'hover:bg-off-white'}
				textColor={isSelected(option[0]) ? 'text-white' : 'text-orange'}
				borderColor={isSelected(option[0]) ? 'border-lighter-green' : 'border-orange'}
			/>
		</button>
	{/each}
</div>
