<script lang="ts">
  import { Input } from '#lib';
  import { goto } from '$app/navigation';
  import { resolve } from '$app/paths';

  let email = $state('');
  let password = $state('');
  let error = $state('');

  async function handleLogin(event: SubmitEvent) {
    event.preventDefault();

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
    }
  }
</script>

<svelte:head>
  <title>Login</title>
</svelte:head>

<main>
  <h1>Sign in</h1>

  <div class="w-full h-full bg-[url('data:image/svg+xml,%3Csvg%20xmlns=%22http://www.w3.org/2000/svg%22%20width=%2220%22%20height=%2220%22%3E%3Cpath%20d=%22M0%200%20L20%200%22%20stroke=%22%23eee%22%20stroke-width=%221%22/%3E%3Cpath%20d=%22M0%200%20L0%2020%22%20stroke=%22%23eee%22%20stroke-width=%221%22/%3E%3C/svg%3E')]">
      <div class="w-full h-full bg-radial from-transparent to-white">
          <div class="flex items-center justify-center h-screen">
              <form class="flex flex-col p-4 rounded-lg gap-y-1 bg-white shadow-xl"
                  onsubmit={handleLogin}
              >
                  <Input name="email"
                      type="text"
                      placeholder='E-Mail'
                  />
                  <Input name="password"
                      type="password"
                      placeholder='Password'
                  />
                  <div class="flex justify-end text-xs text-dark-blue hover:underline">
                      <a href={resolve("/")} class="rounded-sm">
                          "Forgot your password?"
                      </a>
                  </div>
                  <input class="mt-1 text-dark-gray cursor-pointer py-1 bg-blue rounded-sm hover:bg-light-blue"
                      type="submit"
                      value="Submit"
                  />
                  <p>{error}</p>
              </form>
          </div>
      </div>
  </div>
</main>
