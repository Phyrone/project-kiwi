<script lang="ts">
	import { ChevronLeft, ChevronRight } from 'lucide-svelte';

	export let open: boolean = false;
</script>

<nav class="navbar shadow-2xl">
	<div class="navbar-start">
		<button class="btn btn-ghost" on:click|preventDefault|stopPropagation={()=> open = !open}>
			<label class="swap swap-rotate">
				<input type="checkbox" bind:checked={open}>
				<ChevronRight class="swap-off" />
				<ChevronLeft class="swap-on" />
			</label>
		</button>
	</div>

	{open}
</nav>