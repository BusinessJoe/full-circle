<script>
    import { fileToArrayBuffer } from './utils.js';
    export let onSubmit = () => {};
    export let error;
    let filename = "";
    let files = [];

    async function _onSubmit() {
        if (files.length === 0) return;

        filename = files[0].name;
        let buffer = await fileToArrayBuffer(files[0]);
        onSubmit(buffer);
    }
</script>

<div id=input-wrapper>
    <label id=input-label>
        Upload Image
        <input type="file" id=image-input data-testid="image-input" bind:files on:change={_onSubmit}/>
    </label>
    {#if error}
        <span class=error>
            {error}
        </span>
    {:else}
        <span>
            {filename}
        </span>
    {/if}
</div>

<style>
    .error {
        color: red;
    }

    #image-input {
        display: none;
    }

    #input-label {
        background-color: #828282;
        padding: 5px;
        border-radius: 3px;
        display: inline-block;
    }

    #input-label:active {
        transform: scale(0.96);
    }

    #input-wrapper {
        margin-right: 20px;
    }
</style>
