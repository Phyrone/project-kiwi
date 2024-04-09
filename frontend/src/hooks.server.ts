import type { Handle } from '@sveltejs/kit';
import { env } from '$env/dynamic/private';
import { type ExtendedRFC9457Error } from '$lib/RFC9457';

const SESSION_COOKIE_NAME = env.SESSION_COOKIE_NAME || 'session';

export const handle: Handle = async ({ event, event: { cookies, locals }, resolve }) => {
	//locals.session = await get_session_from_request(cookies, request);
	//console.log('locals.session', locals.session);
	locals.session = cookies.get(SESSION_COOKIE_NAME);
	const result = resolve(event);
	if (locals.session && locals.session !== cookies.get(SESSION_COOKIE_NAME)) {
		cookies.set(SESSION_COOKIE_NAME, locals.session, {
			path: '/',
			httpOnly: true,
			secure: true,
			priority: 'high',
			sameSite: 'strict',
			partitioned: true
		});
	}
	return result;
};

export const handleError = async ({ status, message, error, event }) => {
	if (event.url.pathname.startsWith('/_data') || event.locals.json) {
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
