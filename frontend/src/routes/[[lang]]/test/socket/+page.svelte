<script lang="ts">

	import { createSockJs } from '$lib/socket_utils';


	async function start_socket() {
		let socket1 = await createSockJs('http://localhost:7080');
		socket1.send({
			'o': 'a'
		});
	}

</script>
<div>
	<button class="btn" on:click={start_socket}>Connect</button>
</div>
