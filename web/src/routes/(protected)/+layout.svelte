<script lang="ts">
	let { children } = $props();
	import HeaderBar from '#lib/components/HeaderBar.svelte';
	import Navbar from '#lib/components/Navbar.svelte';
</script>

<div class="flex bg-white">
	<Navbar />
	<div class="flex w-full flex-1 flex-col">
		<HeaderBar />
		<main class="m-4 h-full rounded-md bg-off-white p-4 font-serif text-xl">
			{@render children()}
		</main>
	</div>
</div>
