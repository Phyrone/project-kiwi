import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { internalIpV4 } from 'internal-ip';


const is_tauri = !!process.env.TAURI_ENV_PLATFORM;
const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [sveltekit(), purgeCss()],

	envPrefix: ['TAURI_ENV_', 'VITE_'],

	// Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
	//
	// 1. prevent vite from obscuring rust errors
	clearScreen: is_tauri,
	// 2. tauri expects a fixed port, fail if that port is not available
	server: {
		port: 1420,
		strictPort: is_tauri,
		host: mobile ? '0.0.0.0' : false,
		hmr: mobile
			? {
				protocol: 'ws',
				host: await internalIpV4(),
				port: 1421
			}
			: undefined,
		watch: {
			// 3. tell vite to ignore watching `src-tauri`
			ignored: ['**/src-tauri/**']
		}
	},
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	ssr: {
		noExternal: ['three']
	}
}));
