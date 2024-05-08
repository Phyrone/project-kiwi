<script lang="ts">
	import type { LayoutData } from './$types';
	import DefaultAppSkeleton from '$routes/(default)/DefaultAppSkeleton.svelte';

	export let data: LayoutData;

</script>
<DefaultAppSkeleton>
	<slot />
</DefaultAppSkeleton>