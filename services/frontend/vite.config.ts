import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { internalIpV4 } from 'internal-ip';
import topLevelAwait from 'vite-plugin-top-level-await';
import wasm from 'vite-plugin-wasm';
import { Plugin } from 'vite';
import * as fs from 'node:fs';
import * as qs from 'querystring';


const is_tauri = !!process.env.TAURI_ENV_PLATFORM;
const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);


function exportBase64(path: string) {
	const file = fs.readFileSync(path);
	const base64 = file.toString('base64');
	return `export default ${JSON.stringify(base64)}`;
}

function exportDataUrl(path: string, mime?: string) {
	const file = fs.readFileSync(path);
	const base64 = file.toString('base64');
	const dataUrl = `data:${mime ?? 'application/octet-stream'};base64,${base64}`;
	return `export default ${JSON.stringify(dataUrl)}`;
}

const base64Loader: Plugin = {
	name: 'base64-loader',
	transform(src: any, id: string) {
		const [path, query] = id.split('?');
		if (!query) return null;
		const parsed = qs.decode(query);
		if ("base64" in parsed) {
			return exportBase64(path);
		} else if ("dataUrl" in parsed) {
			return exportDataUrl(path, parsed.mime as string);
		} else {
			return null;
		}

	}
};

// https://vitejs.dev/config/
export default defineConfig(async () => ({
	plugins: [
		base64Loader,
		sveltekit(),
		wasm(),
		topLevelAwait(),
		purgeCss()
	],

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
		},
		fs: {
			allow: ['wasm']
		}
	},
	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	ssr: {
		noExternal: ['three']
	}
}));
