import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ fetch, params, params: { guild } }) => {
	const guild_data = fetch(`/api/v1/guilds/${guild}`)
		.then((res) => res.json())
		.catch(() => undefined);

	return {
		guild: await guild_data,
		params
	};
};
