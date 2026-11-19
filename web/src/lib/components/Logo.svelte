<script>
	import { routes } from '#lib/const/routes.ts';
	import { resolve } from '$app/paths';
</script>

<div class="flex justify-center">
	<a href={resolve(routes.home)} class="m-4 rounded-md font-serif text-5xl">
		<span class="text-green">dish</span><span class="text-dark-gray italic">dash</span>
	</a>
</div>
