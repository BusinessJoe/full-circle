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
        <div id=controls>
            <ImagePicker onSubmit={onSubmit} />
            <PlayButton bind:paused={paused} disabled={!image_loaded} />
        </div>
        <Canvas image_width={width} image_height={height} circle_limit={circle_limit} bind:this={canvas} />
        <Timeline max={circle_count} bind:value={circle_limit} />
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
        flex-direction: column;
        height: 90vh;
        width: 90vw;

        justify-content: center;
        align-items: center;
        padding: 0 10px 0 10px;
        background-color: #2b2a33;
        border-radius: 5px;
    }

    #controls {
        width: 100%;

        flex: 1;

        display: flex;
        flex-direction: row;
        justify-content: space-around;
        align-items: center;

        margin: 1em 0 1em 0;
    }

    #main-panel {
    }
</style>
