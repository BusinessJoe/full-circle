<script>
    import Game from '../lib/Game.svelte';

    export let room_id;

    const url = 'ws://' + location.hostname + ':3001/join/' + room_id;

    let name = "";
    let submit = false;
</script>

{#if !submit}
    <form on:submit|preventDefault={() => { submit = true; }}>
        <input bind:value={name} />
        <button type="submit" >
            Go
        </button>
    </form>
{:else}
    <Game websocket_url={url} name={name} />
{/if}
