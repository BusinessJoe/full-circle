<script>
    import { onMount, onDestroy } from 'svelte';

    export let onEvent = () => {}
    export let worker = undefined;

    async function initWebWorker(onEvent) {
        const worker = new Worker('/build/worker.js');
        worker.onmessage = (message) => {
            const { type, payload } = message.data;
            onEvent(worker, type, payload);
        }

        return worker;
    }

    onMount(() => {
    });

    onDestroy(() => {
        worker.terminate();
        worker = undefined;
    });

    const initWasm = () => {
        wasm_bindgen('./pkg/wasm_bg.wasm').then(() => {
            initWebWorker(onEvent).then(new_worker => {
                worker = new_worker;
            });
        });
    }
</script>

<svelte:head>
    <script src="pkg/wasm.js" on:load={initWasm}></script>
</svelte:head>
