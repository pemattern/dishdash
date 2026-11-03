<script lang="ts">
	import { users } from '#lib/api/users.ts';
	import type { UserSchema } from '$bindings';
	import { onMount } from 'svelte';

	let user = $state<UserSchema>();

	onMount(async () => {
		user = await users.me();
	});
</script>

<p>Ingredients</p>
<p>{user?.email}</p>
