<script lang="ts">
	import AppSkeleton from './(default)/AppSkeleton.svelte';
	import { page } from '$app/stores';
</script>

<AppSkeleton>
	<div class="grid flex-auto place-items-center">
		<div class="w-32">
			<h1>{$page.status}</h1>
			<h2>{$page.error.message}</h2>
		</div>
	</div>
</AppSkeleton>
