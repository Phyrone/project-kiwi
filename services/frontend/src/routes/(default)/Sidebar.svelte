<script lang="ts">

	import { Menu } from 'lucide-svelte';

	export let drawer_toggle_id: string;
</script>


<div class="h-full overflow-y-auto flex flex-col w-64">
	<nav class="w-full sticky top-0 z-40 navbar bg-base-100 md:hidden">
		<label for={drawer_toggle_id} class="btn btn-ghost md:disabled drawer-button">
			<Menu />
		</label>
	</nav>
	<ul class="w-full flex-auto menu bg-base-200 text-base-content">

	</ul>
</div>
