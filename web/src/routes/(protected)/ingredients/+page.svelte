<script lang="ts">
	import { users, type GetUserResponse } from "#lib/api/users.ts";
	import { onMount } from "svelte";

	let user = $state<GetUserResponse>();

    onMount(async () => {
      user = await users.me();
    });
</script>

<p>Ingredients</p>
<p>{user?.email}</p>
