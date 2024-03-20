import type { RequestHandler } from './$types';
import { logout_all, SESSION_COOKIE } from '$lib/server/session';
import { json } from '@sveltejs/kit';

export const POST: RequestHandler = async ({ cookies, locals }) => {
	const not_logged_in = !locals.session;
	if (locals.session) {
		await logout_all(locals.session.user_id);
		cookies.delete(SESSION_COOKIE, { path: '/' });
	}

	return json(
		{
			status: not_logged_in ? 'not logged in' : 'ok'
		},
		{
			status: not_logged_in ? 200 : 202
		}
	);
};
