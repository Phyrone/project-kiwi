import type { FetchFunction } from '$types/common';
import type { Writable } from 'svelte/store';
import { browser } from '$app/environment';
import default_phrases_import from '$locales/en.json';
import Polyglot from 'node-polyglot';

export const POLYGOT_CONTEXT_KEY = 'polygot';

const locale_expression = /^([a-z]{2})(?:[-_]([a-z]{2}))?$/;
const filename_to_locale_expression = /^\/src\/locales\/([a-z]{2})(?:[-_]([a-z]{2}))?.json$/;


export type LocaleConfig = Record<string, any> & { locale: string }
const DEFAULT_LOCALE = 'en';

export const DEFAULT_LOCALE_PHRASES: LocaleConfig = {
	locale: DEFAULT_LOCALE,
	...default_phrases_import
};

function fragments_to_locale(lang: string, region: string | undefined): string {
	return `${lang}${region ? '-' + region : ''}`;
}

function convert(
	locales: Record<string, string>
): Record<string, string> {
	return Object.entries(locales).reduce<Record<string, string>>((acc, [path, url]) => {
		const [, locale, region] = filename_to_locale_expression.exec(path.toLowerCase()) || [];
		//rebuild the key
		acc[fragments_to_locale(locale, region)] = url;
		return acc;
	}, {});
}

const locales = convert(import.meta.glob('/src/locales/*.json', {
	eager: true,
	query: '?url',
	import: 'default'
}));


export type ReactivePolygot = Writable<Polyglot>;

export function getAllLocales(): string[] {
	return Object.keys(locales);
}

export function user_lang() {
	if (browser) {
		return navigator.language;
	} else {
		//TODO from headers
		return 'en';
	}
}

export async function findBestPhrases(
	fetch: FetchFunction,
	locale: string | string[] | undefined
): Promise<LocaleConfig> {
	if (!locale) {
		return DEFAULT_LOCALE_PHRASES;
	}
	const locales = Array.isArray(locale) ? locale : [locale];
	for (const single_locale of locales) {
		const [, lang, region] = locale_expression.exec(single_locale.toLowerCase()) ?? [];
		if (!lang) {
			return DEFAULT_LOCALE_PHRASES;
		}
		const polygot = await loadPolygot(fetch, lang, region);
		if (polygot) {
			return polygot;
		}
		const polygot_without_region = await loadPolygot(fetch, lang, undefined);
		if (polygot_without_region) {
			return polygot_without_region;
		}

	}
	return DEFAULT_LOCALE_PHRASES;
}


function get_locale_link(locale: string, region: string | undefined) {
	return locales[fragments_to_locale(locale, region)];
}

export async function loadPolygot(
	fetch: FetchFunction,
	locale: string,
	region: string | undefined
): Promise<LocaleConfig | undefined> {
	const url = get_locale_link(locale, region);
	if (!url) {
		return undefined;
	}
	try {
		const phrases: any = await fetch(url)
			.then((r) => r.ok ? r.json() : Promise.reject(r));

		return {
			locale: fragments_to_locale(locale, region),
			...phrases
		};
	} catch (e) {
		return undefined;
	}
}