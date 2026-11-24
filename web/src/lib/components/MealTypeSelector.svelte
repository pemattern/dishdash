<script lang="ts">
	import { mealTypeLabelMap } from '#lib/const/routes.ts';
	import type { MealTypeCode } from '$bindings';
	import Badge from './Badge.svelte';

	type Props = {
		selected?: MealTypeCode[];
		label?: string;
	};

	let { selected = $bindable<MealTypeCode[]>([]), label }: Props = $props();

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

<div class="flex flex-col">
	{#if label}
		<label for="mealTypeSelector" class="mb-1 text-sm font-bold text-dark-gray">{label}</label>
	{/if}
	<div id="mealTypeSelector" class="flex flex-wrap gap-2" role="group">
		{#each mealTypeLabelMap as option (option)}
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
</div>
