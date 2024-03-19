<script lang="ts">
    export let count = 0;

    let show_indicator = false;
    $: show_indicator = count > 0;
</script>
<button class="btn mx-2" class:indicator={show_indicator}>
    <i class="fa-solid fa-bell"></i>
    {#if show_indicator}
        <span class="indicator-item indicator-bottom badge badge-secondary text-xs">{
            count > 99 ? '99+' : count
        }</span>
    {/if}
</button>