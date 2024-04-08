<script lang="ts">
	const button_count = 100;
	const buttons = Array.from({ length: button_count }, (_, i) => i + 1);
</script>

<div
	class="mx-2 my-2 flex flex-none flex-col overflow-y-auto overflow-x-hidden overscroll-auto rounded-md bg-base-300 px-1 py-2"
>
	{#each buttons as button}
		<a
			class="
                btn btn-circle btn-neutral m-1
                rounded-3xl transition-all
                delay-0 duration-150 ease-in-out will-change-transform
                hover:rounded-lg
            "
			href={'/g/' + button + '/'}>{button}</a
		>
	{/each}
</div>
