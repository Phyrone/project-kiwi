<script lang="ts">
	import { Compass, Home, Menu } from 'lucide-svelte';

	export let drawer_toggle_id: string;
</script>

<nav class="sticky bg-base-200 top-0 navbar md:mb-2 md:rounded-b-md">
	<div class="navbar-start">
		<label for={drawer_toggle_id} class=" md:hidden md:invisible btn btn-ghost md:disabled drawer-button">
			<Menu />
		</label>
		<a href="/" class="btn btn-ghost">
			<Home />
		</a>
	</div>


</nav>