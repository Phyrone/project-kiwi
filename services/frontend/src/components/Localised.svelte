<script lang="ts">


	import { getContext } from 'svelte';
	import { POLYGOT_CONTEXT_KEY, type ReactivePolygot } from '$scripts/localized';
	import WithPolygot from '$components/WithPolygot.svelte';

	const reactive_polygot = getContext<ReactivePolygot>(POLYGOT_CONTEXT_KEY);
	$: polygot = $reactive_polygot;

	export let key: string;
	export let params: any = {};
	export let render: boolean = true;
	export let locale: string = '';
	export let translation: string = '';

	$: translation = polygot.t(key, params);
	$: locale = polygot.locale()

</script>
{#if render}
	{translation}
{/if}



