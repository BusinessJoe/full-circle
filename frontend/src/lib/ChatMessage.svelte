<script>
    export let name;
    export let text;
    export let secret = false;
</script>


<style>
    div {
        overflow-wrap: break-word;
    }

    .secret {
        color: #00FF95;
    }
</style>


<div class:secret={secret}>
    {name}:&nbsp{text}
</div>

