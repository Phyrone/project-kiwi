<script lang="ts">
	import type { Readable } from 'svelte/store';
	import { getContext } from 'svelte';
	import { CONTEXT_CURRENT_LOCALE, CONTEXT_I18N } from '$lib/i18n';
	import type { i18n as I18N } from 'i18next';

	export let key: string;
	export let values: any | undefined = undefined;

	const locale_reactive: Readable<string> = getContext(CONTEXT_CURRENT_LOCALE);
	const i18n = getContext<I18N>(CONTEXT_I18N);
	let locale: string;
	$: locale = $locale_reactive;
	let translated: string;

	$: {
		locale;
		translated = i18n.t(key, values)?.toString();
	}
</script>

<slot {translated}>{translated}</slot>
