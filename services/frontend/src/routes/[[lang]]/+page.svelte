<script lang="ts">
	import { toast } from 'svelte-sonner';
	import { delay } from '$lib/utils';

	async function aFunction() {
		let loading = toast.loading('Loading...', { duration: Infinity });
		try {
			await delay(2000);
			//toast.dismiss(loading);
			toast.success('Success', {
				id: loading,
				duration: 3000,
				description: 'It worked!'
			});
		} catch (e) {
			console.error(e);
			toast.error('Error', {
				id: loading,
				duration: 3000,
				description: e?.toLocaleString()
			});
			throw e;
		}
	}
</script>

<div class="flex-auto">
	<button class="btn btn-primary" on:click|preventDefault={aFunction}>Test</button>
</div>
