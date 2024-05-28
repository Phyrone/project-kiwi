import type { LayoutLoad } from './$types';
import { create_i18n } from '$lib/i18n';
import { writable } from 'svelte/store';
import { browser } from '$app/environment';

export const trailingSlash = 'never';

export const load: LayoutLoad = async ({ data }) => {
	const i18n = await create_i18n({
		browser_lang: browser ? (navigator.languages as string[]) : data.i18n.browser_lang_h
	});

	const current_locale = writable(i18n.language);
	i18n.on('languageChanged', (lng) => {
		current_locale.set(lng);
	});

	return {
		i18n,
		current_locale
	};
};
