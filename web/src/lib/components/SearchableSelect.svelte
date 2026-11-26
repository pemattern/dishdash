<script lang="ts">
	import { onMount } from 'svelte';
	import Label from './Label.svelte';

	type Props = {
		value?: string;
		options?: string[];
		label?: string;
		placeholder?: string;
		required?: boolean;
		disabled?: boolean;
		allowCustom?: boolean;
		name?: string;
	};

	let {
		value = $bindable(''),
		options = [],
		label = '',
		placeholder = 'Search...',
		required = false,
		disabled = false,
		allowCustom = true,
		name
	}: Props = $props();

	let id = $props.id();
	let open = $state(false);
	let search = $state('');
	let activeIndex = $state(0);

	let container: HTMLDivElement;
	let input: HTMLInputElement;

	const filteredOptions = $derived(
		options.filter((option) => option.toLowerCase().includes(search.toLowerCase()))
	);

	function selectOption(option: string) {
		value = option;
		search = '';
		open = false;
		activeIndex = 0;
	}

	function handleInput() {
		open = true;
		activeIndex = 0;

		if (allowCustom) {
			value = search;
		}
	}

	function handleKeydown(event: KeyboardEvent) {
		if (disabled) return;

		switch (event.key) {
			case 'ArrowDown':
				event.preventDefault();
				open = true;
				activeIndex = Math.min(activeIndex + 1, filteredOptions.length - 1);
				break;

			case 'ArrowUp':
				event.preventDefault();
				activeIndex = Math.max(activeIndex - 1, 0);
				break;

			case 'Enter':
				if (open && filteredOptions.length > 0) {
					event.preventDefault();
					selectOption(filteredOptions[activeIndex]);
				} else if (open && allowCustom && search.trim()) {
					selectOption(search.trim());
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
			search = '';
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
		bind:this={input}
		{id}
		{name}
		value={open ? search : value}
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
					aria-selected={i === activeIndex}
					onmousedown={(event) => event.preventDefault()}
					onclick={() => selectOption(option)}
					onmouseenter={() => (activeIndex = i)}
					class="block w-full px-3 py-2 text-left text-sm text-gray hover:bg-off-white"
					class:bg-off-white={i === activeIndex}
				>
					{option}
				</button>
			{/each}
		</div>
	{:else if open && search.trim() && !allowCustom}
		<div
			class="absolute top-full right-0 left-0 z-50 mt-1 rounded-md border border-light-orange bg-white px-3 py-2 text-sm text-gray shadow-lg"
		>
			No options found
		</div>
	{/if}
</div>
