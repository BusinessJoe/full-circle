<script>
    import Game from '../lib/Game.svelte';

    export let room_id;

    const url = 'ws://' + location.hostname + ':3001/join/' + room_id;
</script>

<Game websocket_url={url} />
