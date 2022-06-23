<script context="module">
    export async function initWebWorker(onEvent) {
        const worker = new Worker('/build/worker.js');
        worker.onmessage = (message) => {
            const { type, payload } = message.data;
            onEvent(worker, type, payload);
        }

        return worker;
    }
</script>

<script>
    import { onMount, onDestroy } from 'svelte';

    export let onEvent = () => {}
    export let worker = undefined;

    onMount(() => {
        initWebWorker(onEvent).then(new_worker => {
            worker = new_worker;
        });
    });

    onDestroy(() => {
        worker.terminate();
        worker = undefined;
    });
</script>


