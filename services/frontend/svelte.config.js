import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter_all from '@macfja/svelte-multi-adapter';
import adapter_bun from 'svelte-adapter-bun';
import adapter_static from '@sveltejs/adapter-static';
import adapter_netlify from '@sveltejs/adapter-netlify';
import adapter_vercel from '@sveltejs/adapter-vercel';
import adapter_cloudflare from '@sveltejs/adapter-cloudflare';
import adapter_deno from 'svelte-adapter-deno';

/** @type {import('@sveltejs/kit').Config} */
const config = {
		// Consult https://kit.svelte.dev/docs/integrations#preprocessors
		// for more information about preprocessors
		preprocess: vitePreprocess(),

		kit: {
			// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
			// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
			// See https://kit.svelte.dev/docs/adapters for more information about adapters.
			adapter: adapter_all([
				adapter_bun({
					out: 'out/bun',
					precompress: {
						gzip: true,
						brotli: true
					}
				}),
				adapter_deno({
					out: 'out/deno',
					precompress: true,
				}),
				adapter_static({
					assets: 'out/static',
					pages: 'out/static',
					strict: false,
					precompress: true,
					fallback: 'index.html'
				}),
				adapter_netlify({
					edge: true
				}),
				adapter_vercel({
					runtime: 'edge'
				}),
				adapter_cloudflare()
			]),
			alias: {
				$components: './src/components',
				$lib: './src/lib',
				$styles: './src/styles',
				$scripts: './src/scripts',
				$types: './src/types',
				$assets: './src/assets',
				$routes: './src/routes',
				$locales: './src/locales'
			}
		}
	}
;

export default config;
