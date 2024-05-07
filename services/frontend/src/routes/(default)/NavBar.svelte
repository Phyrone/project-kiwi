<script lang="ts">
	import { Compass, Home, Menu } from 'lucide-svelte';

	export let drawer_toggle_id: string;
</script>

<nav class="sticky bg-base-200 top-0 navbar">
	<div class="navbar-start join">
		<label for={drawer_toggle_id} class="md:hidden btn btn-ghost md:disabled drawer-button">
			<Menu />
		</label>
		<a href="/" class="btn btn-ghost join-item">
			<Home />
		</a>
	</div>


</nav>