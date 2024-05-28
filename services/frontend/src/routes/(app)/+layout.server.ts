import type { LayoutServerLoad } from './$types';
import { parse } from 'accept-language-parser';

export const load: LayoutServerLoad = async ({ request: { headers } }) => {
	const acceptLanguage = headers.get('accept-language');
	const browser_lang_h = acceptLanguage
		? parse(acceptLanguage)
				.sort((a, b) => b.quality - a.quality)
				.map(({ code, region }) => (region ? `${code}-${region}` : code))
		: undefined;
	return {
		i18n: {
			browser_lang_h
		}
	};
};
