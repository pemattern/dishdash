<script lang="ts" generics="T">
	import { onMount } from 'svelte';
	import Label from './Label.svelte';

	type Props = {
		value?: T;
		options?: T[];
		getLabel: (option: T) => string;
		label?: string;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		name?: string;
	};

	let {
		value = $bindable<T>(),
		options = [],
		getLabel,
		label = '',
		placeholder = 'Search...',
		required = false,
		disabled = false,
		name
	}: Props = $props();

	let id = $props.id();
	let open = $state(false);
	let search = $state('');
	let activeIndex = $state(0);

	let container: HTMLDivElement;

	const filteredOptions = $derived(
		options.filter((option) => getLabel(option).toLowerCase().includes(search.toLowerCase()))
	);

	function selectOption(option: T) {
		value = option;
		search = getLabel(option);
		open = false;
		activeIndex = 0;
	}

	function handleInput(event: Event) {
		search = (event.currentTarget as HTMLInputElement).value;
		open = true;
		activeIndex = 0;
	}

	function handleKeydown(event: KeyboardEvent) {
		if (disabled) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				open = true;
				activeIndex = Math.min(activeIndex + 1, Math.max(filteredOptions.length - 1, 0));
				break;

			case 'ArrowUp':
				event.preventDefault();
				activeIndex = Math.max(activeIndex - 1, 0);
				break;

			case 'Enter':
				if (!open) return;

				event.preventDefault();

				if (filteredOptions.length > 0) {
					selectOption(filteredOptions[activeIndex]);
				}
				break;

			case 'Escape':
				open = false;
				search = '';
				break;
		}
	}

	function handleFocus() {
		search = '';
		open = true;
	}

	function handleOutsideClick(event: MouseEvent) {
		if (!container?.contains(event.target as Node)) {
			open = false;
		}
	}

	onMount(() => {
		document.addEventListener('mousedown', handleOutsideClick);

		return () => {
			document.removeEventListener('mousedown', handleOutsideClick);
		};
	});
</script>

<div bind:this={container} class="relative flex min-w-0 flex-1 flex-col">
	{#if label}
		<Label {required} for={id}>{label}</Label>
	{/if}

	<input
		{id}
		{name}
		value={search}
		{placeholder}
		{required}
		{disabled}
		autocomplete="off"
		role="combobox"
		aria-expanded={open}
		aria-controls={`${id}-listbox`}
		aria-autocomplete="list"
		aria-activedescendant={open && filteredOptions.length > 0
			? `${id}-option-${activeIndex}`
			: undefined}
		onfocus={handleFocus}
		oninput={handleInput}
		onkeydown={handleKeydown}
		class="w-full min-w-0 rounded-md border border-light-orange px-3 py-2 text-gray hover:bg-off-white focus:ring-2 focus:ring-light-orange/30 focus:outline-none disabled:cursor-not-allowed disabled:opacity-50"
	/>

	{#if open && filteredOptions.length > 0}
		<div
			id={`${id}-listbox`}
			role="listbox"
			class="absolute top-full right-0 left-0 z-50 mt-1 max-h-56 overflow-y-auto rounded-md border border-light-orange bg-white py-1 shadow-lg"
		>
			{#each filteredOptions as option, i (option)}
				<button
					id={`${id}-option-${i}`}
					type="button"
					role="option"
					aria-selected={option === value}
					onmousedown={(event) => event.preventDefault()}
					onclick={() => selectOption(option)}
					onmouseenter={() => (activeIndex = i)}
					class="block w-full px-3 py-2 text-left text-sm text-gray hover:bg-off-white"
					class:bg-off-white={i === activeIndex}
				>
					{getLabel(option)}
				</button>
			{/each}
		</div>
	{:else if open && search.trim()}
		<div
			class="absolute top-full right-0 left-0 z-50 mt-1 rounded-md border border-light-orange bg-white px-3 py-2 text-sm text-gray shadow-lg"
		>
			No options found
		</div>
	{/if}
</div>
