<script>
    export let max;
    export let value = 0;

    function setValue(val) {
        value = val;
    }

    function pause() {
        console.log('pause pls');
    }

    $: setValue(max);
</script>

<input type=range min=0 max={max} class=timeline bind:value={value} on:input={pause} >

<style>
    .timeline {
        width: 100%;
    }
</style>
