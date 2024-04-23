import type { PageLoad } from './$types';
import { parse_user_selector } from '$lib/selectos';
import { error } from '@sveltejs/kit';

export const load: PageLoad = async ({ params }) => {

	const parsed_user = parse_user_selector('@' + params.user);

	if (!parsed_user) {
		error(400, 'incorrect user selector');
	}

	return {
		parsed_user,
		...params
	};
};
