import type { LayoutLoad } from './$types';
import Polyglot from 'node-polyglot';
import { findBestPhrases } from '$scripts/localized';
import { browser } from '$app/environment';

export const trailingSlash = 'never';

export const load: LayoutLoad = async ({ fetch }) => {
	let locale: string | string[]
	if (browser) {
		locale = navigator.language;
	}else {
		locale = await fetch('/api/app/locale')
			.then((res) => res.ok ? res.json() : undefined)
	}
	const config = await findBestPhrases(fetch, locale);

	const polygot = new Polyglot({
		locale: config.locale,
		phrases: config
	});
	return { polygot };
};