<script>


    import NotificationBell from "./NotificationBell.svelte";
    import TopNavSearchBar from "./TopNavSearchBar.svelte";
</script>

<nav class="bg-base-300 my-2 flex px-2 py-1 rounded-md align-middle">

    <div>
        <a class="btn btn-ghost" href="/">
            <i class="fa-solid fa-compass"/>
        </a>
    </div>
    <div class="flex-auto"><!-- mid filler --> </div>
    <TopNavSearchBar />
    <div class="flex-1"><!-- mid filler --> </div>
    <div>
        <NotificationBell count={2000}/>
        <a class="btn btn-ghost" href="/profile/">
            <i class="fa-solid fa-user"/>
        </a>
    </div>
</nav>