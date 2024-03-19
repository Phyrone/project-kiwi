<script lang="ts">
    const button_count = 100;
    const buttons = Array.from({length: button_count}, (_, i) => i + 1);


</script>


<div class="flex-none my-2 mx-2 py-2 px-1 rounded-md bg-base-300 overflow-y-auto overflow-x-hidden overscroll-auto flex flex-col">
    {#each buttons as button}

        <a class="
                btn btn-circle btn-neutral rounded-3xl
                delay-0 duration-150
                will-change-transform m-1 transition-all hover:rounded-lg
                ease-in-out
            " href={'/channels/_:'+button+'/'}

        >{button}</a>
    {/each}
</div>
