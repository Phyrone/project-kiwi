<script lang="ts">
import { page } from '$app/stores';
import { AppRail, AppRailAnchor } from '@skeletonlabs/skeleton';
import { CircleHelp, Home } from 'lucide-svelte';
</script>
<div class="sticky top-0 h-screen">
	<AppRail width="w-14">
		<AppRailAnchor href="/" selected={$page.url.pathname === '/'}>
			<svelte:fragment slot="lead">
				<Home />
			</svelte:fragment>
		</AppRailAnchor>
		<AppRailAnchor href="/about" selected={$page.url.pathname === '/about'}>
			<svelte:fragment slot="lead">
				<CircleHelp />
			</svelte:fragment>
		</AppRailAnchor>
	</AppRail>
</div>
