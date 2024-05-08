import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vitest/config';
import { purgeCss } from 'vite-plugin-tailwind-purgecss';
import { paraglide } from '@inlang/paraglide-sveltekit/vite';

export default defineConfig({
	plugins: [sveltekit(), purgeCss(), paraglide({
		outdir: 'tmp/paraglide',
		project: 'sveltekit',
	})],

	test: {
		include: ['src/**/*.{test,spec}.{js,ts}']
	},
	ssr: {
		noExternal: ['three']
	}
});
