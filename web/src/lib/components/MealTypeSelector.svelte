<script lang="ts">
	import { MealType } from '$bindings';
	import Badge from './Badge.svelte';

	type Props = {
		selected?: MealType[];
	};

	let { selected = $bindable<MealType[]>([]) }: Props = $props();
	let options = Object.values(MealType) as MealType[];

	function toggle(option: MealType) {
		if (selected.includes(option)) {
			selected = selected.filter((item) => item !== option);
		} else {
			selected.push(option);
		}
	}

	function isSelected(option: MealType) {
		return selected.includes(option);
	}
</script>

<div class="flex flex-wrap gap-2" role="group">
	{#each options as option (option)}
		<button class="rounded-full" type="button" onclick={() => toggle(option)}>
			<Badge
				text={option}
				bgColor={isSelected(option) ? 'bg-vibrant-green' : ''}
				textColor={isSelected(option) ? 'text-white' : 'text-orange'}
				borderColor={isSelected(option) ? 'border-vibrant-green' : 'border-orange'}
			/>
		</button>
	{/each}
</div>
