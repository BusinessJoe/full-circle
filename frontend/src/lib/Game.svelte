<script>
    import { initWebsocket, sendWsEvent } from '../lib/Websocket.svelte';
    import { initWebWorker } from '../lib/WebWorker.svelte';

    export let websocket_url;

    let room_path = "";
    $: short_room_link = room_path ? location.hostname + room_path : "";
    $: room_link = room_path ? location.origin + room_path : "";

    let worker;
    let players = [];
    let player_id;

    $: is_host = new Boolean(players.find(info => info.id === player_id)?.is_host);

    function drawCircle(circle) {
        const canvas = document.getElementById('drawing');
        const ctx = canvas.getContext('2d');

        const { imgx, imgy, center, radius, color } = circle;

        const scale_x = canvas.width / imgx;
        const scale_y = canvas.height / imgy;
        ctx.fillStyle = `rgba(${color[0]},${color[1]},${color[2]},${color[3]/255}`;
        ctx.beginPath();
        ctx.ellipse(
            center[0] * scale_x,
            center[1] * scale_y,
            radius * scale_x,
            radius * scale_y,
            0,
            0,
            2 * Math.PI
        );
        ctx.fill();
    }

    // Websocket event handler
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
                console.log("new canvas dimensions:", width, height);
                canvas.width = width;
                canvas.height = height;
                break;
            case "PlayerList":
                console.log(payload);
                players = payload;
                for (let player of payload) {
                    console.log(player.id);
                }
                break;
            case "PlayerId":
                player_id = payload;
            default:
                console.error(`Type ${type} not recognized`);
                break;
        }
    }

    function onWebWorkerEvent(worker, type, payload) {
        switch (type) {
            case "ready":
                const url = "/moon.jpeg";
                worker.postMessage({ type: "init/url", payload: url });
                break;
            case "init/done":
                const [width, height] = payload;
                sendWsEvent(websocket, "NewImage", { dimensions: [width, height] });
                //canvas.width = width;
                //canvas.height = height;
                //epochBtn.disabled = false;
                break;
            case "epoch/done":
                if (payload) {
                    const { circle, image_data } = payload;
                    // Turn typed arrays (Int32Array, etc.) into normal JS arrays.
                    circle.center = Array.from(circle.center);
                    circle.color = Array.from(circle.color);

                    const message = JSON.stringify({"Circle": circle});
                    sendWsEvent(websocket, "Circle", circle);
                } else {
                    console.log("No circle found");
                }
                worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
                break;
            default:
                console.error(`action type '${type}' not recognized`);
                break;
        }
    }

    let websocket = initWebsocket(websocket_url, onEvent);
    initWebWorker(onWebWorkerEvent).then(_worker => {
        worker = _worker;
    });

    function readSingleFile(e) {
        let file = e.target.files[0];
        if (!file) {
            return;
        }

        file.arrayBuffer().then(buffer => {
            worker.postMessage({ type: "init/buffer", payload: buffer });
        });
    }

    function runEpoch() {
        worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
    }
</script>

<main>
    <h1>Full Circle</h1>
    <a href={room_link}>
        {short_room_link}
    </a>
    <div>
        <button id="start-epochs" on:click={runEpoch} disabled={worker === undefined}>
            Start
        </button>
        <input type="file" id="file-input" on:change={readSingleFile}/>
    </div>
    <div>
        You are {player_id}, host: {is_host}
    </div>
    <div>
        Players:
        {#each players as player}
            <div>
                Name: {player.id}
            </div>
            <div>
                Host: {player.is_host}
            </div>
        {/each}
    </div>
    <canvas id="drawing" />
</main>

<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
            Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
        background-color: #2b2a33;
        color: white;
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
