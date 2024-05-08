<script lang="ts">
	import Sidebar from '$routes/(default)/Sidebar.svelte';
	import NavBar from '$routes/(default)/NavBar.svelte';
</script>


<div class="min-h-screen flex">
	<Sidebar />
	<div class="min-h-screen flex-auto">
		<NavBar/>
		<slot />
	</div>
</div>
