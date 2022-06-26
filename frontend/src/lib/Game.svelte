<script>
    import { onMount, onDestroy } from 'svelte';
    import { initWebsocket, sendWsEvent } from '../lib/Websocket.svelte';
    import WebWorker from '../lib/WebWorker.svelte';
    import PlayerDisplay from '../lib/PlayerDisplay.svelte';
    import ImagePicker from '../lib/ImagePicker.svelte';
    import Chat from '../lib/Chat.svelte';

    export let websocket_url;
    export let name;

    let room_path = "";
    $: short_room_link = room_path ? location.hostname + room_path : "";
    $: room_link = room_path ? location.origin + room_path : "";

    let worker;
    let worker_ready = false;

    let websocket;
    let players = [];
    let messages = [];
    let public_id;
    let width = 100;
    let height = 100;
    let answer = "";
    let answer_hint = "";
    let round_over = true;

    $: is_host = Boolean(players.find(info => info.public_id === public_id)?.is_host);
    $: display_controls = is_host && worker_ready;

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
            case "binary":
                const canvas = document.getElementById("source-image");
                const ctx = canvas.getContext("2d");

                createImageBitmap(payload).then(image => {
                    ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
                });
                console.log('binary', payload);

                console.log("round is over");
                round_over = true;
                break;
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

                answer_hint = payload.answer_hint;
                round_over = false;
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
            case "Answer":
                break;
            case "ChatMessage":
                // We can't just use messages.push(), since the mutation will not trigger an update on its own.
                messages = [...messages, payload];
                break;
            case "ServerMessage":
                messages = [...messages, payload];
                break;
            default:
                console.error(`Type ${type} not recognized`);
                break;
        }
    }

    function onWebWorkerEvent(worker, type, payload) {
        switch (type) {
            case "ready":
                worker_ready = true;
                break;
            case "init/done":
                const [width, height] = payload;
                sendWsEvent(websocket, "NewImage", { dimensions: [width, height], answer });
                runEpoch();
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

    function onSubmit(buf, ans) {
        answer = ans;
        worker.postMessage({ type: "init/buffer", payload: buf });
        console.log('sending binary:', buf);
        websocket.send(buf);
    }

    function runEpoch() {
        worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
    }

    onMount(() => {
        websocket = initWebsocket(websocket_url, name, onEvent);
    });

    onDestroy(() => {
        websocket.close();
    });
</script>


{#if is_host}
    <WebWorker onEvent={onWebWorkerEvent} bind:worker={worker}/>
{/if}

<main>
    <div id="game-wrapper">
        <PlayerDisplay players={players} />
        <div id="main-panel">
            <span class=hint>
                {answer_hint}
            </span>
            <a href={room_link}>
                {short_room_link}
            </a>
            {#if display_controls}
                <ImagePicker onSubmit={onSubmit} />
            {/if}
            <div>
                You are {public_id}, host: {is_host}
            </div>
            <div id="canvas-wrapper" 
                class="canvas-wrapper"
                class:canvas-wrapper-landscape={width > height}
                class:canvas-wrapper-portrait={!(width > height)}
            >
                <canvas id="canvas" width={width} height={height} 
                    class="canvas"
                    class:canvas-landscape={width > height}
                    class:canvas-portrait={!(width > height)}
                />
                <canvas id="source-image" width={width} height={height} 
                    class="canvas"
                    class:fade={round_over}
                    class:hide={!round_over}
                    class:canvas-landscape={width > height}
                    class:canvas-portrait={!(width > height)}
                />
            </div>
        </div>
        <Chat websocket={websocket} messages={messages} />
    </div>
</main>


<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
            Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            background-color: #1e1e1e;
            color: white;
    }

    main {
        display: flex;
        justify-content: center;
    }

    #game-wrapper {
        display: flex;
        height: 90vh;
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

    @keyframes fadeIn {
        0% { opacity: 0; }
        100% { opacity: 1; }
    }
    .fade {
        animation: fadeIn 2.0s;
        transition: opacity 0.3s;
    }

    .fade:hover {
        opacity: 0%;
    }

    .hide {
        visibility: hidden;
    }

    .canvas {
        border: 1px solid white;
        background-color: #000;
        position: absolute;
    }

    .canvas-wrapper {
        position: relative;
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

    .hint {
        letter-spacing: 5px;
        text-align: center;
        width: 100%;
        margin: 1em 0 1em 0;
    }
</style>
