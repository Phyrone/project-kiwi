<script lang="ts">
	import type { LayoutData } from './$types';
	import Sidebar from '$routes/(default)/Sidebar.svelte';
	import NavBar from '$routes/(default)/NavBar.svelte';
	import { setContext } from 'svelte';
	import { writable } from 'svelte/store';
	import { POLYGOT_CONTEXT_KEY, type ReactivePolygot } from '$scripts/localized';

	const drawer_toggle_id = 'my-drawer';
	let drawer = false;
	let clientWidth: number = 0;
	$: if (clientWidth >= 768) drawer = false;

	export let data: LayoutData;
	setContext<ReactivePolygot>(POLYGOT_CONTEXT_KEY, writable(data.polygot));

</script>
<svelte:window bind:innerWidth={clientWidth} />
<div
	class="min-h-screen drawer overscroll-y-none transition md:drawer-open md:max-w-screen-xl overflow-clip mx-auto">
	<input bind:checked={drawer} id={drawer_toggle_id} type="checkbox" class="drawer-toggle" />
	<div class="drawer-content md:mx-4">
		<!-- Page content here -->
		<NavBar {drawer_toggle_id} />
		<slot />
	</div>
	<div class="drawer-side">
		<label for={drawer_toggle_id} aria-label="close sidebar" class="drawer-overlay"></label>
		<Sidebar {drawer_toggle_id} />
	</div>
</div>
