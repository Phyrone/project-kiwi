<script lang="ts">
	import type { LayoutData } from './$types';
	import DefaultAppSkeleton from '$routes/(app)/DefaultAppSkeleton.svelte';
	import { setContext } from 'svelte';
	import { CONTEXT_CURRENT_LOCALE, CONTEXT_I18N } from '$lib/i18n';
	import { browser } from '$app/environment';

	export let data: LayoutData;
	const current_locale = data.current_locale;
	const i18n = data.i18n;

	setContext(CONTEXT_I18N, i18n);
	setContext(CONTEXT_CURRENT_LOCALE, current_locale);

	$: if (browser) {
		console.log('[i18n] Current locale:', $current_locale);
	}
</script>

<DefaultAppSkeleton>
	<slot />
</DefaultAppSkeleton>
