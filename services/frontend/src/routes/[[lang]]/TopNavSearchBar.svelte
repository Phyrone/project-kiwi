<script lang="ts">
	let search_query = '';
</script>

<label class="input input-bordered flex flex-auto items-center">
	<i class="fa-solid fa-magnifying-glass opacity-70" />
	<input type="text" placeholder="Search" class="w-full border-0" bind:value={search_query} />
	<!-- <kbd class="kbd kbd-sm select-none">⌘</kbd> -->
	<!-- <kbd class="kbd kbd-sm select-none">F</kbd> -->
</label>
