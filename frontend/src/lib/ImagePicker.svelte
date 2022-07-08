<script>
    import { fileToArrayBuffer } from './utils.js';
    export let onSubmit = () => {};
    let files = [];

    async function _onSubmit() {
        if (files.length === 0) return;

        let buffer = await fileToArrayBuffer(files[0]);
        onSubmit(buffer);
    }
</script>

<input type="file" data-testid="image-input" bind:files on:change={_onSubmit}/>

