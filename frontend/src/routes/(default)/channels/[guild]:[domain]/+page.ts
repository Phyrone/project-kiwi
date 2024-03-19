import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params: { guild, domain }, fetch }) => {
	return {
		props: {
			guild,
			domain
		}
	};
};
