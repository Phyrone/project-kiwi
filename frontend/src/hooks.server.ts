import type { Handle } from '@sveltejs/kit';

import { type ExtendedRFC9457Error } from '$lib/RFC9457';
import { get_session_from_request } from '$lib/server/session';

export const handle: Handle = async ({ event, event: { locals, request, cookies }, resolve }) => {
	locals.session = await get_session_from_request(cookies, request);
	//console.log('locals.session', locals.session);

	return resolve(event);
};

export const handleError = async ({ status, message, error, event }) => {
	if (event.url.pathname.startsWith('/api') || event.locals.json) {
		const response = errorMessageByCode(status, message, event.url.pathname);
		if (response) {
			console.error('unhandled error', error);
			event.setHeaders({
				'Content-Type': 'application/problem+json',
				'Content-Language': 'en'
			});
			// @ts-ignore
			return response;
		} else {
			return;
		}
	} else {
		console.error('unhandled error', error);
		return;
	}
};

function errorMessageByCode(
	code: number,
	message: string,
	path: string
): ExtendedRFC9457Error | undefined {
	let part: {
		type: string;
		title: string;
	};
	switch (true) {
		case code >= 500:
			part = {
				type: 'https://www.rfc-editor.org/rfc/rfc9110.html#name-500-internal-server-error',
				title: 'Internal Server Error'
			};
			break;
		case code == 400:
			part = {
				type: 'https://www.rfc-editor.org/rfc/rfc9110.html#name-400-bad-request',
				title: 'Bad Request'
			};
			break;
		case code == 404:
			part = {
				type: 'https://www.rfc-editor.org/rfc/rfc9110.html#name-404-not-found',
				title: 'Not Found'
			};
			break;

		default:
			return undefined;
	}

	return {
		success: false,
		detail: message,
		instance: path,
		status: code,
		...part
	};
}
