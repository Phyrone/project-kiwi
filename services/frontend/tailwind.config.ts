import { skeleton } from '@skeletonlabs/tw-plugin';
import { join } from 'path';

/** @type {import('tailwindcss').Config} */
export default {
	darkMode: 'media',
	content: [
		'./src/**/*.{html,js,svelte,ts}',
		// 3. Append the path to the Skeleton package
		join(require.resolve(
				'@skeletonlabs/skeleton'),
			'../**/*.{html,js,svelte,ts}'
		)
	],
	theme: {
		extend: {},
	},
	plugins: [
		skeleton({
			themes: {
				preset: [
					'skeleton',
					'modern',
					'sahara',
					'vintage',
					'crimson',
					'wintry'
				]
			}
		})
	]
};
