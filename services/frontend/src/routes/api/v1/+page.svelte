<script lang="ts">
	import 'swagger-ui/dist/swagger-ui.css';
	import SwaggerUI from 'swagger-ui';
	import { onMount } from 'svelte';

	onMount(() => {
		SwaggerUI({
			dom_id: '#swagger',
			url: '/api/v1/schema.yaml'
		});
	});
</script>

<div id="swagger" />
