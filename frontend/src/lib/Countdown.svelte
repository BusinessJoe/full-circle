<script>
    import { onMount, onDestroy } from 'svelte';
    export let websocket;
    export let enabled;
    let countdown;

    onMount(() => {
        websocket.addEventListener("Countdown", (payload) => {
            countdown = payload;
        });
    });
</script>

{#if enabled}
    {countdown}
{/if}
