import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter_all from '@macfja/svelte-multi-adapter';
import adapter_bun from 'svelte-adapter-bun';
import adapter_static from '@sveltejs/adapter-static';
import adapter_netlify from '@sveltejs/adapter-netlify';
import adapter_vercel from '@sveltejs/adapter-vercel';
import adapter_cloudflare from '@sveltejs/adapter-cloudflare';

const is_tauri = !!process.env.TAURI_ENV_PLATFORM;
const build_target = process.env.BUILD_TARGET;

/** @return {import('@sveltejs/kit').Config} */
function get_adapter(){
	if(is_tauri){
		return adapter_static({
			//precompression is contraproductive for tauri since ships multiple versions of the same file
			//and tauri will be shipped will all files and does not lazy load them on demand
			//so compression would make the app bigger
			//tauri will compress the files itself anways
			precompress:false,
			strict: false,
			fallback: 'index.html',
			assets: 'dist',
			pages: 'dist'
		})
	}else {
		switch (build_target?.toLowerCase()){
			case 'netlify':
				return adapter_netlify({
					edge: true
				})
			case 'vercel':
				return adapter_vercel({
					runtime: 'edge'
				})
			case 'cloudflare':
				return adapter_cloudflare({
					fallback: 'spa',
				})
			case 'spa':
				return adapter_static({
					assets: 'dist',
					pages: 'dist',
					strict: false,
					precompress: true,
					fallback: 'index.html'
				})
			case 'bun':
			case 'default':
			default:
				return adapter_bun({
					out: 'dist',
					precompress: {
						gzip: true,
						brotli: true
					},

				});
		}
	}
}


/** @type {import('@sveltejs/kit').Config} */
const config = {
	// Consult https://kit.svelte.dev/docs/integrations#preprocessors
	// for more information about preprocessors
	preprocess: vitePreprocess(),

	kit: {
		// adapter-auto only supports some environments, see https://kit.svelte.dev/docs/adapter-auto for a list.
		// If your environment is not supported, or you settled on a specific environment, switch out the adapter.
		// See https://kit.svelte.dev/docs/adapters for more information about adapters.
		adapter: get_adapter(),
		alias: {
			$components: './src/components',
			$lib: './src/lib',
			$styles: './src/styles',
			$types: './src/types',
			$assets: './src/dicebear',
			$routes: './src/routes',
			$locales: './src/locales'
		}
	}
};
export default config;
