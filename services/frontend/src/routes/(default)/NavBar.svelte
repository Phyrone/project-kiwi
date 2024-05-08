<script lang="ts">
	import { CompassIcon, HomeIcon } from 'lucide-svelte';
	import { AppBar } from '@skeletonlabs/skeleton';
</script>
<div class="sticky top-0">
	<AppBar padding="p-1.5" gridColumns="grid-cols-3" slotDefault="place-self-center" slotTrail="place-content-end">
		<svelte:fragment slot="lead">
			<div>
				<a class="btn" href="/">
					<HomeIcon />
				</a>
				<a class="btn" href="/discover">
					<CompassIcon />
				</a>
			</div>
		</svelte:fragment>

		<svelte:fragment slot="trail">

		</svelte:fragment>
	</AppBar>
</div>
