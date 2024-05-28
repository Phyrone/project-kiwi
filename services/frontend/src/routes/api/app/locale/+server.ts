import { text } from '@sveltejs/kit';
import type { RequestHandler } from './$types';
import { parse, pick } from 'accept-language-parser';
import { locales } from '$lib/i18n';

export const GET: RequestHandler = async ({ request: { headers } }) => {
	const acceptLanguage = headers.get('accept-language') || '';
	const browser_supported = parse(acceptLanguage);
	const suggestion = pick(locales, browser_supported);

	return text(suggestion || 'en');
};
