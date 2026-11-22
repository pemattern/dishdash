import { paraglideVitePlugin } from '@inlang/paraglide-js';
import tailwindcss from '@tailwindcss/vite';
import adapter from '@sveltejs/adapter-static';
import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';
import { fileURLToPath } from 'node:url';

export default defineConfig({
	plugins: [
		tailwindcss(),
		sveltekit({
			compilerOptions: {
				// Force runes mode for the project, except for libraries. Can be removed in svelte 6.
				runes: ({ filename }) =>
					filename.split(/[/\\]/).includes('node_modules') ? undefined : true
			},
			adapter: adapter({ pages: 'build', assets: 'build', fallback: 'index.html' })
		}),
		paraglideVitePlugin({
			project: './project.inlang',
			outdir: './src/lib/paraglide',
			emitTsDeclarations: true,
			strategy: ['localStorage', 'preferredLanguage', 'baseLocale']
		})
	],
	resolve: {
		alias: {
			$bindings: fileURLToPath(new URL('../bindings', import.meta.url))
		}
	},
	server: {
		proxy: {
			'/api': 'http://0.0.0.0:3000',
			'/data': 'http://0.0.0.0:3000'
		}
	}
});
