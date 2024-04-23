<script lang="ts">
	import 'dropzone/src/dropzone.scss';
	import Dropzone from 'dropzone';
	import { onDestroy } from 'svelte';

	Dropzone.autoDiscover = false;

	let dropzone: Dropzone;

	function setup_dropzone(element: HTMLElement) {
		dropzone = new Dropzone(element, {
			url: '/api/v1/media/upload',
			method: 'PUT',
			uploadMultiple: false,
			chunking: false,
			parallelChunkUploads: false,
			paramName: 'file'
		});
	}

	onDestroy(() => {
		if (dropzone) {
			dropzone.destroy();
		}
	});
</script>

<div class="h-full w-full">
	<div class="dropzone h-full" use:setup_dropzone />
</div>
