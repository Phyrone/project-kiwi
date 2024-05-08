<script lang="ts">
	import '$styles/app.scss';
	import { Toaster } from 'svelte-french-toast';
	import { autoModeWatcher } from '@skeletonlabs/skeleton';

	const themes = [
		'skeleton',
		'modern',
		'sahara',
		'vintage',
		'crimson',
		'wintry'
	];
</script>

<!-- code is trusted -->
<!-- eslint-disable svelte/no-at-html-tags -->
<svelte:head>
	{@html '<script>(' + autoModeWatcher.toString() + ')();</script>'}
</svelte:head>

<Toaster />
<slot />
