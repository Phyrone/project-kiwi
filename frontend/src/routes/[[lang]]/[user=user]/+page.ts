import type { PageLoad } from '../../../../.svelte-kit/types/src/routes';
import { parse_user_selector } from '$lib/selectos';

export const load: PageLoad = async ({ params }) => {
	const parsed_user = parse_user_selector(params.user);

	return {
		parsed_user,
		...params
	};
};
