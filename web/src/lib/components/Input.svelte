<script>
	let {
		label = '',
		placeholder = '',
		type = 'text',
		value = $bindable(''),
		name = undefined
	} = $props();
</script>

<div class="flex flex-col">
	{#if label}
		<label class="text-sm font-bold text-dark-gray" for="name">
			{label}
		</label>
	{/if}
	<input
		id={name}
		{name}
		{type}
		{placeholder}
		bind:value
		class="mt-1 rounded-md border border-gray px-2 py-1"
	/>
</div>
