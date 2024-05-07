<script lang="ts">
	import Sidebar from '$routes/(default)/Sidebar.svelte';
	import NavBar from '$routes/(default)/NavBar.svelte';

	const drawer_toggle_id = 'my-drawer';
	let drawer = false;
	let clientWidth: number = 0;
	$: if (clientWidth >= 768) drawer = false;

</script>
<svelte:window bind:innerWidth={clientWidth} />
<div
	class="min-h-screen drawer overscroll-y-none transition md:drawer-open md:max-w-screen-xl xl:rounded-xl mx-auto shadow-2xl">
	<input bind:checked={drawer} id={drawer_toggle_id} type="checkbox" class="drawer-toggle" />
	<div class="drawer-content">
		<!-- Page content here -->
		<NavBar {drawer_toggle_id} />
		<slot />
	</div>
	<div class="drawer-side">
		<label for={drawer_toggle_id} aria-label="close sidebar" class="drawer-overlay"></label>
		<Sidebar {drawer_toggle_id} {drawer} />
	</div>
</div>
