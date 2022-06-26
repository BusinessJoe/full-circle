<script>
    import Game from '../lib/Game.svelte';

    export let room_id;

    const ws_url = 'ws://' + location.hostname + ':3001/join/' + room_id;
    const api_origin = 'http://' + location.hostname + ':3001';

    let name = "";
    let submit = false;
</script>

{#if !submit}
    <form on:submit|preventDefault={() => { submit = true; }}>
        <input bind:value={name} maxlength={20} />
        <button type="submit" >
            Go
        </button>
    </form>
{:else}
    <Game room_id={room_id} api_origin={api_origin} websocket_url={ws_url} name={name} />
{/if}
