import { redirect } from '@sveltejs/kit';

export const trailingSlash = 'ignore';

export const GET = async () => {
	redirect(302, '/api/v1/');
};
