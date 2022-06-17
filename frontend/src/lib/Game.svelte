<script>
    import { onMount, onDestroy } from 'svelte';
    import { initWebsocket, sendWsEvent } from '../lib/Websocket.svelte';
    import { initWebWorker } from '../lib/WebWorker.svelte';
    import PlayerDisplay from '../lib/PlayerDisplay.svelte';

    export let websocket_url;
    export let name;

    let room_path = "";
    $: short_room_link = room_path ? location.hostname + room_path : "";
    $: room_link = room_path ? location.origin + room_path : "";

    let worker;
    let websocket;
    let players = [];
    let public_id;
    let is_host = false;
    let width = 100;
    let height = 100;

    $: is_host = Boolean(players.find(info => info.public_id === public_id)?.is_host);

    function drawCircle(circle) {
        const canvas = document.getElementById('canvas');
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
        console.log("Event:", type);
        switch (type) {
            case "Circle":
                drawCircle(payload);
                break;
            case "CircleSequence":
                for (let circle of payload) {
                    drawCircle(circle);
                }
                break;
            case "RoomPath":
                room_path = payload;
                break;
            case "NewImage":
                const [new_width, new_height] = payload.dimensions;
                width = new_width;
                height = new_height;
                console.log("new canvas dimensions:", width, height);
                break;
            case "PrivateInfo":
                console.log(payload);
                public_id = payload.info.public_id;
                break;
            case "PlayerList":
                players = payload;
                break;
            case "Host":
                is_host = payload;
                break;
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

    onMount(() => {
        websocket = initWebsocket(websocket_url, name, onEvent);

        initWebWorker(onWebWorkerEvent).then(_worker => {
            worker = _worker;
        });
    });

    onDestroy(() => {
        websocket.close();
        worker.terminate();
    });
</script>

<main>
    <div id="game-wrapper">
        <div id="main-panel">
            <h1>Full Circle</h1>
            <a href={room_link}>
                {short_room_link}
            </a>
            <div>
                <button id="start-epochs" on:click={runEpoch} disabled={!is_host || (worker === undefined)}>
                    Start
                </button>
                <input type="file" id="file-input" on:change={readSingleFile} disabled={!is_host || (worker === undefined)} />
            </div>
            <div>
                You are {public_id}, host: {is_host}
            </div>
            <div id="canvas-wrapper" class={width > height ? "canvas-wrapper-landscape" : "canvas-wrapper-portrait"}>
                <canvas id="canvas" width={width} height={height} class={width > height ? "canvas-landscape" : "canvas-portrait"} />
            </div>
        </div>
            <PlayerDisplay players={players} />
    </div>
</main>

<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
            Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            background-color: #1e1e1e;
            color: white;
    }

    h1 {
        color: #ff3e00;
        text-transform: uppercase;
        font-size: 2rem;
        font-weight: 100;
        line-height: 1.1;
        margin: 0.5rem 0;
        max-width: 14rem;
    }

    main {
        display: flex;
        justify-content: center;
    }

    #game-wrapper {
        display: flex;
    }

    #main-panel {
        display: flex;
        flex-direction: column;
        justify-content: center;
        padding: 10px;
        background-color: #2b2a33;
        border-radius: 5px;
    }

    #canvas-wrapper {
        width: 70vh;
        height: 70vh;
        display: flex;
        justify-content: center;
    }

    #canvas {
        border: 1px solid white;
    }

    .canvas-wrapper-landscape {
        flex-direction: column;
    }

    .canvas-landscape {
        width: 100%;
        height: auto;
    }

    .canvas-wrapper-portrait {
        flex-direction: row;
    }

    .canvas-portrait {
        height: 100%;
        width: auto;
        margin-left: auto;
        margin-right: auto;
    }
</style>
