<script lang="ts">
	import './layout.css';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
</script>

<svelte:head>
	<meta name="sveltekit:prefetch" content="off" />
</svelte:head>
{@render children()}
