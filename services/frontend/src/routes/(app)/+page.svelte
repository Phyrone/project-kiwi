<script>
	import toast from 'svelte-french-toast';
	import { IS_TAURI, TAURI_PLATFORM } from '$lib/env';

	function handleClick() {
		toast.success('Hello world!');
	}
</script>
<p>tauri: {IS_TAURI}</p>
<button class="btn btn-primary" on:click={handleClick}> Test Toast </button>
<input class="input">
