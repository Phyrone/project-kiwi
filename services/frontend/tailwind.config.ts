import daisyuiPlugin from 'daisyui';
import * as containerQueryPlugin from '@tailwindcss/container-queries';
import safeAreaPlugin from 'tailwindcss-safe-area';

/** @type {import('tailwindcss').Config} */
export default {
	darkMode: 'media',
	content: ['./src/**/*.{html,js,svelte,ts}'],
	theme: {
		extend: {
			width: {
				sim: '4rem'
			},
			containers: {
				sim: '4.1rem'
			}
		}
	},
	plugins: [daisyuiPlugin, containerQueryPlugin, safeAreaPlugin]
};
