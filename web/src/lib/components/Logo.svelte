<script>
	import { routes } from '#lib';
	import { resolve } from '$app/paths';
</script>

<div class="flex justify-center">
	<a href={resolve(routes.home)} class="m-4 rounded-md text-2xl">
		<span class="text-dark-blue">dish</span>
		<span class="dark-gray">dash</span>
	</a>
</div>
