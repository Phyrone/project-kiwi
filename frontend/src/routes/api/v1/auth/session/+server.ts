import type { RequestHandler } from './$types';
import { json } from '@sveltejs/kit';
import { standardizedError } from '$lib/RFC9457';
import { DOCUMENTATION_API_ERRORS_URL } from '$lib/consts';

export const GET: RequestHandler = async ({ locals }) => {
	if (locals.session) {
		let issuedAt: Date | null | string = null;
		if (locals.session.iat) {
			issuedAt = new Date(locals.session.iat * 1000).toISOString();
		}

		return json({
			user_id: locals.session.user_id,
			issued_at: issuedAt
		});
	} else {
		return standardizedError({
			type: DOCUMENTATION_API_ERRORS_URL + '/auth#not-logged-in',
			title: 'Not logged in',
			detail: 'You are not logged in.',
			status: 400
		});
	}
};
