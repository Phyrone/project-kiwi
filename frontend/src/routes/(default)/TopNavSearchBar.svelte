<script lang="ts">

    let search_query = ''

</script>

<label class="flex-auto flex items-center input input-bordered">
    <i class="opacity-70 fa-solid fa-magnifying-glass"/>
    <input type="text" placeholder="Search" class="w-full border-0" bind:value={search_query}>
    <!-- <kbd class="kbd kbd-sm select-none">⌘</kbd> -->
    <!-- <kbd class="kbd kbd-sm select-none">F</kbd> -->
</label>