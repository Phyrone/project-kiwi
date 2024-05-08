<script lang="ts">

	import {  } from 'qr-code-styling/package.json';
	import { onMount } from 'svelte';

	export let content: string;
	let qr = new QRCodeStyling({
		width: 300,
		height: 300,
		data: content,
	})

	onMount(() =>{


	})
</script>