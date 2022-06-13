<script>
    import logo from './assets/svelte.png';
    import Counter from './lib/Counter.svelte';
    import { initWebsocket, sendWsEvent } from './lib/Websocket.svelte';

    import init, { TestStruct } from 'wasm';
    init().then(() => {
        console.log("inited");
    });

    const worker = new Worker(new URL('./worker.js', import.meta.url));
    worker.onmessage = (message) => {
        console.log(message.data);
        const { type, payload } = message.data;
        switch (type) {
            case "ready":
                worker.postMessage({ type: "init/url", payload: "/moon.jpeg" });
                break;
            case "init/done":
                worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
                break;
        }
    }

    let room_path = "";
    $: short_room_link = room_path ? location.hostname + room_path : "";
    $: room_link = room_path ? location.origin + room_path : "";

    function onEvent(type, payload) {
        switch (type) {
            case "Circle":
                drawCircle(payload);
                break;
            case "RoomPath":
                room_path = payload;
                break;
            case "NewImage":
                const canvas = document.getElementById('drawing');
                const [width, height] = payload.dimensions;
                canvas.width = width;
                canvas.height = height;
                break;
            case "PlayerList":
                console.log(payload);
                for (let player of payload) {
                    console.log(player.id);
                }
                break;
            default:
                console.error(`Type ${type} not recognized`);
                break;
        }
    }

    let title = "Full Circle";

    const uri = 'ws://' + location.hostname + ':3001' + '/room';
    initWebsocket(uri, onEvent);
</script>

<main>
    <h1>{title}</h1>
    <a href={room_link}>
        {short_room_link}
    </a>
    <div>
        <button id="start-epochs">
            Start
        </button>
        <input type="file" id="file-input" />
    </div>
    <canvas id="drawing" />
</main>

<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
            Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
    }

    main {
        margin: 1em;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 2rem;
        font-weight: 100;
        line-height: 1.1;
        margin: 1rem 0;
        max-width: 14rem;
    }
</style>
