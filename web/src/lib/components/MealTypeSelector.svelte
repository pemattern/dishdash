<script lang="ts">
	import { MealType } from '$bindings';

	type Props = {
		selected?: MealType[];
	};

	let { selected = $bindable<MealType[]>([]) }: Props = $props();
	let options = Object.values(MealType) as MealType[];

	function toggle(option: MealType) {
		if (selected.includes(option)) {
			selected = selected.filter((item) => item !== option);
		}
	}
</script>

<div class="flex flex-wrap gap-2" role="group">
	{#each options as option (option)}
		<button
			type="button"
			aria-pressed={selected.includes(option)}
			onclick={() => toggle(option)}
			class="
          focus-visible:ring-green-500 rounded-full border px-3 py-1.5 text-sm
          font-medium transition-colors
          duration-150 focus-visible:ring-2
          focus-visible:ring-offset-2 focus-visible:outline-none
        "
			class:bg-green-100={selected.includes(option)}
			class:border-green-300={selected.includes(option)}
			class:text-green-800={selected.includes(option)}
			class:bg-white={!selected.includes(option)}
			class:border-gray-300={!selected.includes(option)}
			class:text-gray-700={!selected.includes(option)}
			class:hover:bg-gray-50={!selected.includes(option)}
		>
			{option}
		</button>
	{/each}
</div>
