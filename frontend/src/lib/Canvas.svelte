<script>
    import { onMount, onDestroy } from 'svelte';
    export let width;
    export let height;
    export let circle_limit;

    let circles = [];

    function drawCircle(ctx, circle) {

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

    function drawCircles(circle_limit, circles) {
        const canvas = document.getElementById('canvas');
        if (canvas) {
            const ctx = canvas.getContext('2d');
            ctx.clearRect(0, 0, canvas.width, canvas.height);

            for (let i = 0; i < circle_limit; i++) {
                drawCircle(ctx, circles[i]);
            }
        }
    }

    export function addCircle(circle) {
        circles = [...circles, circle];
        circle_limit = circles.length;
    }

    $: landscape = width > height;

    $: drawCircles(circle_limit, circles);
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
</div>

<style>
    #canvas-wrapper {
        width: min(70vh, 90vw);
        height: min(70vh, 90vw);
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
