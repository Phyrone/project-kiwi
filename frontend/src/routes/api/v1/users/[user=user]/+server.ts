import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { sql } from '$lib/server/database';
import { parse_user_selector } from '$lib/selectos';

export const GET: RequestHandler = async ({ params: { user: unparsed_user } }) => {
	const user = parse_user_selector(unparsed_user);

	return json({
		status: 'just a test',
		user: user
		//data: rows[0].session_secret
	});
};
