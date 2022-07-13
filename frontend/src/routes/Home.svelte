<script>
import { onMount, onDestroy } from 'svelte';
import WebWorker from '../lib/WebWorker.svelte';
import ImagePicker from '../lib/ImagePicker.svelte';
import Canvas from '../lib/Canvas.svelte';
import Timeline from '../lib/Timeline.svelte';
import PlayButton from '../lib/PlayButton.svelte';
import { arrayBufferToBase64 } from '../lib/utils.js';

let worker;
let worker_ready = false;
let image_loaded = false;
let paused = true;
let epoch_in_progress = false;

let canvas;

let load_error = "";

let width = 100;
let height = 100;

let buffer = null;

let epoch_count = 0;
let circle_count = 0;
let num_generations = 30;

let circle_limit;



function onWebWorkerEvent(worker, type, payload) {
    switch (type) {
        case "ready":
            worker_ready = true;
            break;
        case "init/done":
            const [new_width, new_height] = payload;
            width = new_width;
            height = new_height;
            image_loaded = true;
            break;
        case "init/error":
            console.error(payload);
            load_error = "Failed to load image";
            break;
        case "epoch/done":
            epoch_in_progress = false;
            epoch_count++;
            if (payload) {
                circle_count++;
                const { circle, image_data } = payload;
                // Turn typed arrays (Int32Array, etc.) into normal JS arrays.
                circle.center = Array.from(circle.center);
                circle.color = Array.from(circle.color);

                canvas.addCircle(circle);
            } 

            if (!paused) {
                runEpoch();
            }
            break;
        default:
            console.error(`action type '${type}' not recognized`);
            break;
    }
}

function onSubmit(buf) {
    load_error = "";
    worker.postMessage({ type: "init/buffer", payload: buf });
}

function runEpoch() {
    epoch_in_progress = true;
    worker.postMessage({ type: "epoch", payload: { num_gens: num_generations, gen_size: 100 } });
}


$: if (image_loaded && !paused && !epoch_in_progress) {
    runEpoch();
}
</script>


<WebWorker onEvent={onWebWorkerEvent} bind:worker={worker}/>

<main>
    <div id=game-wrapper>
        <div id=controls class=paper>
            <ImagePicker onSubmit={onSubmit} error={load_error} />
            <PlayButton bind:paused={paused} disabled={!image_loaded} />
        </div>
        <Canvas image_width={width} image_height={height} circle_limit={circle_limit} bind:this={canvas} />
        <div id=timeline-wrapper class=paper>
            <Timeline max={circle_count} bind:value={circle_limit} />
        </div>
    </div>
</main>


<style>
    :root {
        font-family: -apple-system, BlinkMacSystemFont, 'Segoe UI', Roboto, Oxygen,
            Ubuntu, Cantarell, 'Open Sans', 'Helvetica Neue', sans-serif;
            background-color: #1e1e1e;
            color: white;
    }

    :global(body) {
        margin: 0;
    }

    main {
        height: 100vh;
        width: 100%;
        padding: 8px;
        box-sizing: border-box;
        display: flex;
        justify-content: center;
    }


    #game-wrapper {
        display: flex;
        flex-direction: column;
        gap: 10px;
        justify-content: center;
        align-items: center;

        height: 100%;
        width: 90%;

        padding: 0 10px 0 10px;
    }

    #controls {
        max-width: 50%;

        flex: 1;

        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;

        padding-left: 20px;
        padding-right: 20px;
    }

    #timeline-wrapper {
        width: 35vw;
        padding-left: 20px;
        padding-right: 20px;
    }

    .paper {
        background-color: #2b2a33;
        border-radius: 5px;
        padding: 5px;
    }
</style>
