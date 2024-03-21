<script>

	const test_items = Array.from({ length: 250 }, (v, k) => k);
</script>


<h1>Settings</h1>
{#each test_items as item}
	<p>{item}</p>
{/each}