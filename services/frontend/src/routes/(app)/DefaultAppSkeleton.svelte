<script lang="ts">

	import MenuWraper from '$components/MenuWraper.svelte';
	import SideMenu from '$routes/(app)/SideMenu.svelte';
</script>

<MenuWraper>
	<svelte:fragment let:open slot="default">
		<p>open: {open}</p>
		<slot />
	</svelte:fragment>
	<svelte:fragment slot="menu">
		<SideMenu />
	</svelte:fragment>
</MenuWraper>
