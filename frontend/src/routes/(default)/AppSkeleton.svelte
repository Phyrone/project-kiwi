<script>
	import Sidebar from './Sidebar.svelte';
	import TopNavbar from './TopNavbar.svelte';
</script>

<div class="flex h-screen overflow-hidden">
	<Sidebar />
	<div class="ml-0 mr-2 flex flex-auto flex-col">
		<TopNavbar />
		<div class="mb-2 flex flex-auto">
			<slot />
		</div>
	</div>
</div>
