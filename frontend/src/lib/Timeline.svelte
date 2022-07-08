<script>
    export let max;
    export let value = 0;

    let grabbed = false;

    function setValue(val) {
        // When the user is grabbing (moving) the slider, we should
        // avoid changing its value.
        if (!grabbed) {
            value = val;
        }
    }

    function handleGrab() {
        grabbed = true;
    }

    function handleRelease() {
        grabbed = false;
    }

    $: setValue(max);
</script>

<input type=range min=0 max={max} class=timeline bind:value={value} on:input={handleGrab} on:change={handleRelease} >

<style>
    .timeline {
        width: 100%;
        flex: 1;
    }
</style>
