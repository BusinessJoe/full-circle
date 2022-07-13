<script>
    import { onMount, onDestroy } from 'svelte';
    export let image_width;
    export let image_height;
    export let circle_limit;

    let canvas;
    let wrapper;

    let wrapper_dims;
    $: aspect_ratio = image_width / image_height;
    let virtual_width = 1000;
    $: virtual_height = virtual_width / aspect_ratio;
    let narrower;
    $: if (wrapper_dims) {
        narrower = aspect_ratio < (wrapper_dims.width / wrapper_dims.height);
    }


    let circles = [];


    const resizeObserver = new ResizeObserver(entries => {
        for (let entry of entries) {
            wrapper_dims = { 
                width: entry.contentBoxSize[0].inlineSize, 
                height: entry.contentBoxSize[0].blockSize,
            };
        }
    });

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
    }

    $: landscape = image_width > image_height;

    $: drawCircles(circle_limit, circles);

    onMount(() => {
        resizeObserver.observe(wrapper);
    });
</script>

<div id=canvas-wrapper 
    class=canvas-wrapper
    bind:this={wrapper}
>
    <canvas id=canvas width={virtual_width} height={virtual_height} 
        class=canvas
        class:width-limited={!narrower}
        class:height-limited={narrower}
        bind:this={canvas}
    />
</div>

<style>
    #canvas-wrapper {
        width: 90%;
        height: 90%;

        display: flex;
        justify-content: center;
    }

    .canvas {
        position: absolute;
        border: 1px solid white;
        background-color: #000;
    }

    .canvas-wrapper {
        position: relative;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }

    .width-limited {
        width: 100%;
        height: auto;
    }

    .height-limited {
        height: 100%;
        width: auto;
    }
</style>
