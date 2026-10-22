<script lang="ts">
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  let email = $state('');
  let password = $state('');
  let error = $state('');
  let loading = $state(false);

  async function handleLogin(event: SubmitEvent) {
    event.preventDefault();
    error = '';
    loading = true;

    try {
      const response = await fetch('/api/login', {
        method: 'POST',
        headers: {
          'Content-Type': 'application/json'
        },
        credentials: 'include',
        body: JSON.stringify({ email, password })
      });

      if (!response.ok) {
        throw new Error('Invalid email or password');
      }

      await goto(resolve('/'));
    } catch {
      error = 'Login failed. Please check your credentials.';
    } finally {
      loading = false;
    }
  }
</script>

<svelte:head>
  <title>Login</title>
</svelte:head>

<main>
  <h1>Sign in</h1>

  <form onsubmit={handleLogin}>
    <label for="email">Email</label>
    <input
      id="email"
      type="email"
      bind:value={email}
      autocomplete="username"
      required
    />

    <label for="password">Password</label>
    <input
      id="password"
      type="password"
      bind:value={password}
      autocomplete="current-password"
      required
    />

    {#if error}
      <p role="alert">{error}</p>
    {/if}

    <button type="submit" disabled={loading}>
      {loading ? 'Signing in…' : 'Sign in'}
    </button>
  </form>
</main>
