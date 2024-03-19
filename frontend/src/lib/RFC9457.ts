import { json } from '@sveltejs/kit';
import { DOCUMENTATION_API_ERRORS_URL } from '$lib/consts';

export type ErrorDescription = {
	type: string;
	title: string;
	detail: string;

	instance?: string;
	status?: number;
	additional?: Record<string, unknown>;
};

export type RFC9457Error = {
	type: string;
	title: string;
	detail: string;
	instance?: string;
} & Record<string, unknown>;

export type ExtendedRFC9457Error = RFC9457Error & {
	success: false;
};

/**
 * Responds with an error as specified in RFC 9457 (https://tools.ietf.org/html/rfc9457)
 *
 * @param description A description of the error containing all necessary information specified in RFC 9457
 */
export function standardizedError(description: ErrorDescription): Response {
	return json(
		{
			success: false,
			type: description.type,
			title: description.title,
			detail: description.detail,
			instance: description.instance,
			...description.additional
		} satisfies ExtendedRFC9457Error,
		{
			headers: {
				'Content-Type': 'application/problem+json',
				Accept: 'application/json, application/problem+json',
				'Content-Language': 'en'
			},
			status: description.status ?? 500
		}
	);
}

export function respond_known_errors(error: unknown): Response {
	if (typeof error !== 'object' || error === null) {
		throw error;
	}
	if (error instanceof Error) {
		let description: ErrorDescription;
		switch (error.name) {
			case 'ValidationError':
				description = {
					type: `${DOCUMENTATION_API_ERRORS_URL}common#request-schema`,
					title: 'Invalid Schema',
					detail: error.message,
					status: 400,
					additional: {
						// @ts-ignore
						report: error.details
					}
				};
				break;
			case 'SyntaxError':
				description = {
					type: `${DOCUMENTATION_API_ERRORS_URL}common#invalid-json`,
					title: 'Invalid JSON',
					detail: error.message,
					status: 400
				};
				break;
			default:
				throw error;
		}
		return standardizedError(description);
	} else {
		throw error;
	}
}
