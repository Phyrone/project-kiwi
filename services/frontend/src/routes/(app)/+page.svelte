<script lang="ts">
	import toast from 'svelte-french-toast';
	import { IS_TAURI } from '$lib/env';
	import Hls from 'hls.js';
	import worker_url from 'hls.js/dist/hls.worker.js?worker&url';

	function handleClick() {
		toast.success('Hello world!');
	}

	function onVideoCreate(element: HTMLVideoElement) {
		if (Hls.isSupported()) {
			let hls = new Hls();
			hls.config.workerPath = worker_url;
			hls.config.enableWorker = true;
			hls.attachMedia(element);
			hls.on(Hls.Events.ERROR,(event, data) => {
				console.error('HLS: Error', event, data);
			});
			hls.loadSource('/some-video.m3u8');
		} else {
			alert('HLS not supported');
		}


	}
</script>
<p>tauri: {IS_TAURI}</p>
<button class="btn btn-primary" on:click={handleClick}> Test Toast</button>
<input class="input">

<video
	class="w-96 h-fit"
	controls
	use:onVideoCreate
></video>