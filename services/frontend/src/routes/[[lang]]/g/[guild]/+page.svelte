<script lang="ts">
	import type { PageData } from './$types';

	export let data: PageData;
</script>

<div class="mx-auto h-full w-full max-w-screen-md rounded-md bg-green-50">
	<h1>{JSON.stringify(data)}</h1>
	ABC
</div>
