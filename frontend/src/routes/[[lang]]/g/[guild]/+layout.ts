import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params }) => {
	return {
		//guild: await guild_data,
		params
	};
};
