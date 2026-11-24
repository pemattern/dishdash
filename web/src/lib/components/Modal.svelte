<script lang="ts">
	import { onMount, type Snippet } from 'svelte';

	let {
		open = $bindable(),
		children
	}: {
		open?: boolean;
		children: Snippet;
	} = $props();

	onMount(() => {
		const handleKeydown = (event: KeyboardEvent) => {
			if (event.key === 'Escape') {
				open = false;
			}
		};
		window.addEventListener('keydown', handleKeydown);
		return () => {
			window.removeEventListener('keydown', handleKeydown);
		};
	});
</script>

{#if open}
	<div
		class="fixed inset-0 z-50 grid place-items-center overflow-y-auto bg-black/50 p-4"
		role="presentation"
		onclick={(event) => {
			if (event.target === event.currentTarget) {
				open = false;
			}
		}}
	>
		<section class="w-fit max-w-xl overflow-y-auto rounded-xl bg-white text-dark-gray shadow-2xl">
			<div class="p-4">
				{@render children()}
			</div>
		</section>
	</div>
{/if}
