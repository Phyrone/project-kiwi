<script lang="ts">
	import type { PageData } from './$types';
	import Localised from '$components/Localised.svelte';

	export let data: PageData;
</script>

<h1>
	<Localised
		key="test"
		values={{
			name: data.user
		}}
	/>
</h1>
