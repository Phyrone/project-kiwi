<script lang="ts">
	import type { LayoutData } from './$types';
	import GuildSkeleton from './GuildSkeleton.svelte';
	//import { setContext } from 'svelte';

	export let data: LayoutData;
</script>

<GuildSkeleton>
	<slot />
</GuildSkeleton>
