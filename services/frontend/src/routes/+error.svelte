<script>
	import DefaultAppSkeleton from '$routes/(default)/DefaultAppSkeleton.svelte';
	import { page } from '$app/stores';
</script>

<DefaultAppSkeleton>
	<h1>{$page.status}</h1>
	<h2>{$page.error?.message}</h2>
</DefaultAppSkeleton>