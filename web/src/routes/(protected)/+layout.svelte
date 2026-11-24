<script lang="ts">
	let { children } = $props();
	import HeaderBar from '#lib/components/HeaderBar.svelte';
	import Navbar from '#lib/components/Navbar.svelte';
</script>

<div class="flex h-screen overflow-hidden bg-off-white">
	<Navbar />
	<div class="flex min-h-0 w-full flex-1 flex-col">
		<HeaderBar />
		<main class="scrollbar-hide m-4 min-h-0 flex-1 overflow-y-auto rounded-md bg-white p-4">
			{@render children()}
		</main>
	</div>
</div>
