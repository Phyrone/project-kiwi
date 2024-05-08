<script lang="ts">
	import { page } from '$app/stores';
	import type { LayoutData } from './$types';
	import { setContext } from 'svelte';
	import { writable } from 'svelte/store';
	import { POLYGOT_CONTEXT_KEY, type ReactivePolygot } from '$scripts/localized';
	import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';
	import DefaultAppSkeleton from '$routes/(default)/DefaultAppSkeleton.svelte';

	export let data: LayoutData;
	setContext<ReactivePolygot>(POLYGOT_CONTEXT_KEY, writable(data.polygot));

</script>
<DefaultAppSkeleton>
	<slot/>
</DefaultAppSkeleton>