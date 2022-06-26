<script>
    import { fileToArrayBuffer } from './utils.js';
    export let onSubmit = () => {};
    let files = [];
    let name = "";
    let show = true;

    async function _onSubmit() {
        if (files.length === 0 || name === "") return;

        let buffer = await fileToArrayBuffer(files[0]);
        onSubmit(buffer, name);
        show = false;
    }
</script>

{#if show}
    <input type="file" data-testid="image-input" bind:files/>
    <input type="text" data-testid="answer-input" bind:value={name} maxlength="20" />

    <button data-testid="submit-button" on:click={_onSubmit}>
        Ok
    </button>
{/if}
