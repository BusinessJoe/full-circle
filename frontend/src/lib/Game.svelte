<script context="module">
    export const GameStates = {
        InProgress: "InProgress",
        RoundOver: "RoundOver",
    };
</script>

<script>
import { onMount, onDestroy } from 'svelte';
import { PlayerWebSocket } from '../lib/Websocket.svelte';
import WebWorker from '../lib/WebWorker.svelte';
import PlayerDisplay from '../lib/PlayerDisplay.svelte';
import ImagePicker from '../lib/ImagePicker.svelte';
import Chat from '../lib/Chat.svelte';
import Countdown from '../lib/Countdown.svelte';
import Canvas from '../lib/Canvas.svelte';
import { arrayBufferToBase64 } from '../lib/utils.js';

export let room_id;
export let websocket_url;
export let api_origin;
export let name;

let worker;
let worker_ready = false;

let websocket = new PlayerWebSocket(websocket_url, name);
let players = [];
let public_id;
let private_id;
let width = 100;
let height = 100;

let answer = "";
let buffer = null;

let answer_hint = "";
let game_state = GameStates.RoundOver;

let epoch_count = 0;
let circle_count = 0;
let num_generations = 100;

$: is_host = Boolean(players.find(info => info.public_id === public_id)?.is_host);


function onWebWorkerEvent(worker, type, payload) {
    switch (type) {
        case "ready":
            worker_ready = true;
            break;
        case "init/done":
            const [width, height] = payload;

            let image_data = arrayBufferToBase64(buffer);
            let image_dimensions = [width, height];
            fetch(`${api_origin}/image`, {
                method: 'POST',
                headers: {
                    "room-id": room_id,
                    "private-id": private_id,
                    "content-type": "application/json",
                },
                body: JSON.stringify({
                    image_data,
                    image_dimensions,
                    answer,
                }),
            });
            console.log('sent image data');

            break;
        case "epoch/done":
            epoch_count++;
            if (payload) {
                circle_count++;
                const { circle, image_data } = payload;
                // Turn typed arrays (Int32Array, etc.) into normal JS arrays.
                circle.center = Array.from(circle.center);
                circle.color = Array.from(circle.color);

                const message = JSON.stringify({"Circle": circle});
                websocket.send("Circle", circle);
            } 

            runEpoch();
            break;
        default:
            console.error(`action type '${type}' not recognized`);
            break;
    }
}

function onSubmit(buf, ans) {
    answer = ans;
    buffer = buf;
    worker.postMessage({ type: "init/buffer", payload: buf });
}

function runEpoch() {
    worker.postMessage({ type: "epoch", payload: { num_gens: num_generations, gen_size: 100 } });
}

onMount(() => {
    websocket.addEventListener("binary", (payload) => {
        console.log("round is over");
        game_state = GameStates.RoundOver;
    });
    websocket.addEventListener("NewImage", (payload) => {
        answer_hint = payload.answer_hint;
        game_state = GameStates.InProgress;
        if (is_host) {
            runEpoch();
        }
    });
    websocket.addEventListener("PrivateInfo", (payload) => {
        public_id = payload.info.public_id;
        private_id = payload.private_id;
    });
    websocket.addEventListener("PlayerList", (payload) => {
        players = payload;
    });
    websocket.addEventListener("Host", (payload) => {
        is_host = payload;
    });
});

onDestroy(() => {
    websocket.close();
});

function handlePass() {
    websocket.send("Pass");
}

function handleGiveUp() {
    websocket.send("GiveUp");
}
</script>


{#if is_host}
    <WebWorker onEvent={onWebWorkerEvent} bind:worker={worker}/>
{/if}

<main>
    <div id="game-wrapper">
        <div class=panel>
            <PlayerDisplay players={players} />
            {#if is_host}
                <button on:click={handlePass}>Pass</button>
            {:else}
                <button on:click={handleGiveUp}>Give Up</button>
            {/if}
        </div>
        <div class=panel id="main-panel">
            {#if answer_hint}
                <span class=hint>
                    {answer_hint}
                </span>
            {/if}
            {#if game_state === GameStates.InProgress}
                <Countdown websocket={websocket} enabled={true} />
            {/if}
            {#if is_host && game_state === GameStates.InProgress}
                {circle_count}/{epoch_count}
            {/if}
            {#if is_host && worker_ready && game_state === GameStates.RoundOver}
                <ImagePicker onSubmit={onSubmit} />
            {/if}
            <div>
                You are {public_id}, host: {is_host}
            </div>
            <Canvas websocket={websocket} width={width} height={height} game_state={game_state}/>
        </div>
        <Chat websocket={websocket} />
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

    .panel {
        display: flex;
        flex-direction: column;
    }

    #main-panel {
        justify-content: center;
        padding: 10px;
        background-color: #2b2a33;
        border-radius: 5px;
    }

    .hint {
        letter-spacing: 5px;
        text-align: center;
        width: 100%;
        margin: 1em 0 1em 0;
    }
</style>
