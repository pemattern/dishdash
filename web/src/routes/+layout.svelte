<script lang="ts">
	import './layout.css';
	import type { LayoutProps } from './$types';

	let { children }: LayoutProps = $props();
</script>

<div
	class="h-full w-full bg-[url('data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20width=%2220%22%20height=%2220%22%3E%3Cpath%20d=%22M0%200%20L20%200%22%20stroke=%22%23eee%22%20stroke-width=%221%22/%3E%3Cpath%20d=%22M0%200%20L0%2020%22%20stroke=%22%23eee%22%20stroke-width=%221%22/%3E%3C/svg%3E')]"
>

	<div class="h-full w-full bg-radial from-transparent to-white">
	{@render children()}
	</div>
</div>
