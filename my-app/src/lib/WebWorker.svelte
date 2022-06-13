<script context="module">
    export async function initWebWorker() {
        await wasm_bindgen('js/pkg/wasm_bg.wasm');
    }
</script>
