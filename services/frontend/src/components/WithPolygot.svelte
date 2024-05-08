<script lang="ts">
	import { getContext } from 'svelte';
	import { POLYGOT_CONTEXT_KEY, type ReactivePolygot } from '$scripts/localized';
	const reactive_polygot = getContext<ReactivePolygot>(POLYGOT_CONTEXT_KEY);
	$: polygot = $reactive_polygot;

</script>
<slot {polygot}/>