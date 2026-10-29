<script>
	import { routes } from '#lib/const/routes.ts';
	import { resolve } from '$app/paths';
</script>

<div class="flex justify-center">
	<a href={resolve(routes.home)} class="m-4 rounded-md font-serif text-4xl">
		<span class="text-dark-blue">dish</span>
		<span class="text-dark-gray">dash</span>
	</a>
</div>
