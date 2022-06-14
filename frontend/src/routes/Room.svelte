<script>
    import { navigate } from "svelte-routing";
    async function create_room() {
        const api_host = location.hostname + ':3001';
        const url = 'http://' + api_host + '/room';
        const res = await fetch(url, {
            method: 'GET',
        });

        return await res.text();
    }

    //create_room().then(room_id => {
    //    navigate(room_id);
    //});
</script>

<button on:click={() => create_room().then(navigate)}>
    Create Room
</button>
