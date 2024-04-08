<script lang="ts">
	import { getContext } from 'svelte';

	const user = getContext('user');
</script>

<h1>TODO post</h1>
<p>{JSON.stringify(user)}</p>
