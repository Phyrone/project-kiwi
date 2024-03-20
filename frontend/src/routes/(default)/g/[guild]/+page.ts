import type { PageLoad } from './$types';

export const load: PageLoad = async ({ params, parent }) => {

	const parent_data = parent();

	return {
		params,
		guild: parent_data.then((p) => p.guild)
	};
};
