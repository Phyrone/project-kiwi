// See https://kit.svelte.dev/docs/types#app
// for information about these interfaces
import type { SessionData } from '$lib/server/auth';

declare global {
	namespace App {
		// interface Error {}
		interface Locals {
			session: SessionData | undefined;
			json?: boolean;
		}

		// interface PageData {}
		// interface PageState {}
		// interface Platform {}
		interface Error {}
	}
}

export {};
