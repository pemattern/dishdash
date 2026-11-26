<script lang="ts">
	import type { Snippet } from 'svelte';

	const { onclick = () => {}, children }: { onclick?: () => void; children: Snippet } = $props();
</script>

<button
	{onclick}
	type="button"
	class="pointer-events-auto z-10 flex cursor-pointer items-center justify-center rounded-md bg-white p-2 text-dark-green opacity-0 group-hover:opacity-100 hover:bg-off-white focus-visible:opacity-100"
>
	{@render children()}
</button>
