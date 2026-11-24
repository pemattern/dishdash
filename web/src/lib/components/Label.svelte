<script lang="ts">
	import type { Snippet } from 'svelte';

	let {
		for: htmlFor,
		required = false,
		children
	}: {
		for: string;
		required?: boolean;
		children: Snippet;
	} = $props();
</script>

<div class="flex flex-col gap-1.5">
	<label for={htmlFor} class="mb-1 text-sm font-medium text-dark-gray">
		{@render children()}
		{#if required}
			<span class="ml-0.5 text-orange" aria-hidden="true">*</span>
		{/if}
	</label>
</div>
