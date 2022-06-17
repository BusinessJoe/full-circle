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
