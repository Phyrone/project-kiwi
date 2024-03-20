import type { LayoutLoad } from './$types';

export const load: LayoutLoad = async ({ params: { user }, fetch }) => {
	const about_user_request = fetch('/api/v1/users/' + user).then((res) => res.json());

	return {
		user: await about_user_request
	};
};
