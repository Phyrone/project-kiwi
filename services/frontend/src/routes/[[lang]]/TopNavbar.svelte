<script>
	import NotificationBell from './NotificationBell.svelte';
	import TopNavSearchBar from './TopNavSearchBar.svelte';
	import ProfileDropdown from './ProfileDropdown.svelte';
</script>

<nav class="my-2 flex rounded-md bg-base-300 px-2 py-1 align-middle">
	<div>
		<a class="btn btn-primary mr-1 w-32" href="/">
			<i class="fa-solid fa-compass text-xl" />
		</a>
		<a class="btn" href="/new/">
			<i class="fa-solid fa-plus-circle" />
		</a>
	</div>
	<div class="flex-auto"><!-- mid filler --></div>
	<TopNavSearchBar />
	<div class="flex-1"><!-- mid filler --></div>
	<div>
		<NotificationBell count={0} />
		<ProfileDropdown />
	</div>
</nav>
