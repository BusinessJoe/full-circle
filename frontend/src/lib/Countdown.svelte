<script>
    import { onMount, onDestroy } from 'svelte';
    export let websocket;
    export let enabled;
    let countdown = undefined;

    onMount(() => {
        websocket.addEventListener("Countdown", (payload) => {
            countdown = payload;
        });
    });
</script>

{#if enabled && countdown !== undefined}
    {countdown}
{/if}
