<script>
	import NotificationBell from './NotificationBell.svelte';
	import TopNavSearchBar from './TopNavSearchBar.svelte';
</script>

<nav class="my-2 flex rounded-md bg-base-300 px-2 py-1 align-middle">
	<div>
		<a class="btn btn-ghost" href="/">
			<i class="fa-solid fa-compass" />
		</a>
	</div>
	<div class="flex-auto"><!-- mid filler --></div>
	<TopNavSearchBar />
	<div class="flex-1"><!-- mid filler --></div>
	<div>
		<NotificationBell count={2000} />
		<a class="btn btn-ghost" href="/settings/">
			<i class="fa-solid fa-user" />
		</a>
	</div>
</nav>
