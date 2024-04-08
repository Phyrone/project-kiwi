import type { LayoutLoad } from './$types';
export const trailingSlash = 'never';

export const load: LayoutLoad = async ({ fetch }) => {
	//const session_data = fetch('/api/v1/auth/session').then((res) => res.json());


	return {
		//session: await session_data
	};
};