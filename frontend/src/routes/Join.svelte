<script>
    export let room_id;

    import { initWebsocket, sendWsEvent } from '../lib/Websocket.svelte';
    import { initWebWorker } from '../lib/WebWorker.svelte';

    console.log(room_id);

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
                for (let player of payload) {
                    console.log(player.id);
                }
                break;
            default:
                console.error(`Type ${type} not recognized`);
                break;
        }
    }

    const uri = 'ws://' + location.hostname + ':3001/join/' + room_id;
    let websocket = initWebsocket(uri, onEvent);
</script>


<main>
    <h1>{room_id}</h1>
    <div>
        <input type="file" id="file-input" />
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
