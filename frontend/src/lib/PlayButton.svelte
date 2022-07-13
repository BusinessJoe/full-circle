<script>
    export let disabled;
    export let paused = true;

    let base_play_color = "rgb(37, 196, 47)";
    let disabled_play_color = "rgb(130, 130, 130)";
    $: play_color = disabled ? disabled_play_color : base_play_color;

    let pause_color = "rgb(234, 155, 44)";

    document.addEventListener('keypress', event => {
        if (event.key === " " && !disabled) {
            event.preventDefault();
            paused = !paused;
        }
    });
    // Hacky way to prevent spacebar from clicking focused buttons
    document.addEventListener('keyup', event => {
        if (event.key === " " && !disabled) {
            event.preventDefault();
        }
    });
</script>

<!--
<button id=play-toggle class:play-icon={paused} class:pause-icon={!paused} on:click={() => paused = !paused} disabled={disabled}>
</button>
-->
<button id=play-toggle on:click={() => paused = !paused} disabled={disabled}>
    {#if paused}
        <svg viewBox="248.889 138.222 56.889 64.593" xmlns="http://www.w3.org/2000/svg" xmlns:bx="https://boxy-svg.com">
            <path d="M 277.334 142.074 L 309.63 198.963 L 245.037 198.963 L 277.334 142.074 Z" style={`fill: ${play_color};`} transform="matrix(0, 1, -1, 0, 447.851996, -106.815004)" bx:shape="triangle 245.037 142.074 64.593 56.889 0.5 0 1@bcf35f3f"/>
        </svg>
    {:else}
        <svg viewBox="200 100 73.481 74.667" xmlns="http://www.w3.org/2000/svg">
            <rect x="200" y="100" width="28.444" height="74.667" style={`fill: ${pause_color};`}/>
            <rect x="245.037" y="100" width="28.444" height="74.667" style={`fill: ${pause_color};`}/>
        </svg>
    {/if}
</button>

<style>
    #play-toggle {
        background-repeat: no-repeat;
        background-size: auto, 20px;
        background-position: center;
        width: 40px;
        height: 40px;
        border: none;
        background-color: transparent;
    }

    #play-toggle:active {
        transform: scale(0.96);
    }
</style>
