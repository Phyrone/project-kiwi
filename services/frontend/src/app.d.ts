import type { i18n as I18N } from 'i18next';

// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			i18n: I18N;
		}

		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
	}
}

export {};
