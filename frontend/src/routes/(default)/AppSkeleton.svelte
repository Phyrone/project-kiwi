<script>
    import Sidebar from "./Sidebar.svelte";
    import TopNavbar from "./TopNavbar.svelte";
</script>

<div class="flex h-screen overflow-hidden">
    <Sidebar/>
    <div class="flex-auto mr-2 ml-0 flex flex-col">
        <TopNavbar/>
        <div class="flex-auto flex mb-2">
            <slot/>
        </div>
    </div>
</div>