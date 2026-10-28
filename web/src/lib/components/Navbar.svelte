<script lang="ts">
	import { routes } from '#lib';
	import { resolve } from '$app/paths';
	import { page } from '$app/state';
	import Logo from './Logo.svelte';
	import { Apple, ListTodo, Soup, type LucideIcon } from '@lucide/svelte';

	type NavbarEntry = {
		path: string;
		text: string;
		icon: LucideIcon;
	};

	type NavbarGroup = {
		header: string;
		entries: NavbarEntry[];
	};

	const navbarData = [
		{
			header: 'Cooking',
			entries: [
				{ path: routes.recipes, text: 'Recipes', icon: Soup },
				{ path: routes.ingredients, text: 'Ingredients', icon: Apple }
			]
		},
		{
			header: 'Shopping',
			entries: [{ path: routes.shoppingLists, text: 'Shopping Lists', icon: ListTodo }]
		}
	] satisfies NavbarGroup[];

	function isCurrent(path: string): boolean {
		return page.url.pathname === path;
	}
</script>

<nav class="h-screen w-56">
	<Logo />
	<div class="flex flex-col gap-y-2">
		{#each navbarData as group (group.header)}
			<div class="pl-4">
				<div class="pb-1 text-sm text-dark-gray">{group.header}</div>
				<div class="flex flex-col gap-y-2">
					{#each group.entries as entry (entry.text)}
						<a
							href={resolve(entry.path)}
							class={`text-md rounded-md border hover:border-gray hover:bg-white ${isCurrent(resolve(entry.path)) ? 'border-gray bg-white text-dark-blue' : 'border-transparent'}`}
						>
							<div class="ml-2 flex w-fit items-center gap-4 p-2">
								<entry.icon size={22} stroke-width={1.75} />
								{entry.text}
							</div>
						</a>
					{/each}
				</div>
			</div>
		{/each}
	</div>
</nav>
