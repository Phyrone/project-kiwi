import { vitePreprocess } from '@sveltejs/vite-plugin-svelte';
import adapter_bun from 'svelte-adapter-bun';
import adapter_static from '@sveltejs/adapter-static';

const is_tauri = !!process.env.TAURI_ENV_PLATFORM;
const build_target = process.env.BUILD_TARGET;

/** @return {import('@sveltejs/kit').Config} */
function get_adapter() {
	if (is_tauri) {
		return adapter_static({
			//tauri will compress the files itself if necessary
			precompress: false,
			strict: false,
			fallback: 'index.html',
			assets: 'dist',
			pages: 'dist'
		});
	} else {
		switch (build_target?.toLowerCase()) {
			case 'bun':
			case 'default':
			default:
				return adapter_bun({
					out: 'dist',
					precompress: {
						gzip: true,
						brotli: true
					}

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
			$assets: './src/assets',
			$routes: './src/routes',
			$locales: './src/locales',
			$grpc: './src/grpc'
		}
	}
};
export default config;
