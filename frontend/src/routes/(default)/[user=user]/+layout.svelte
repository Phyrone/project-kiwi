<script lang="ts">
	import type { PageData } from './$types';
	import { setContext } from 'svelte';

	export let data: PageData;
	setContext('user', data.user);
</script>

<slot />
