import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params: { guild, channel, domain }, fetch }) => {
	return {
		props: {
			guild,
			channel,
			domain
		}
	};
};
