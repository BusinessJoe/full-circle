<script>
    import { onMount, onDestroy } from 'svelte';
    import { GameStates } from './Game.svelte';
    export let websocket;
    export let width;
    export let height;
    export let game_state;

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

    onMount(() => {
        websocket.addEventListener("binary", (payload) => {
            const canvas = document.getElementById("source-image");
            const ctx = canvas.getContext("2d");

            createImageBitmap(payload).then(image => {
                ctx.drawImage(image, 0, 0, canvas.width, canvas.height);
            });
        });
        websocket.addEventListener("Circle", (payload) => {
            drawCircle(payload);
        });
        websocket.addEventListener("CircleSequence", (payload) => {
            for (let circle of payload) {
                drawCircle(circle);
            }
        });
        websocket.addEventListener("NewImage", (payload) => {
            const [new_width, new_height] = payload.dimensions;
            width = new_width;
            height = new_height;
            console.log("new canvas dimensions:", width, height);
        });
    });

    $: landscape = width > height;
</script>

<div id="canvas-wrapper" 
    class="canvas-wrapper"
    class:canvas-wrapper-landscape={landscape}
    class:canvas-wrapper-portrait={!landscape}
>
    <canvas id="canvas" width={width} height={height} 
        class="canvas"
        class:canvas-landscape={landscape}
        class:canvas-portrait={!landscape}
    />
    <canvas id="source-image" width={width} height={height} 
        class="canvas"
        class:fade={game_state === GameStates.RoundOver}
        class:hide={game_state !== GameStates.RoundOver}
        class:canvas-landscape={landscape}
        class:canvas-portrait={!landscape}
    />
</div>

<style>
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
</style>
