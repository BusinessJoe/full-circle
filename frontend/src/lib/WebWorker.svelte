<script context="module">
    export async function initWebWorker(onEvent) {
        const worker = new Worker('/build/worker.js');
        worker.onmessage = (message) => {
            console.log(message.data);
            const { type, payload } = message.data;
            console.log("processing worker event", type);
            onEvent(worker, type, payload);
        }

        return worker;
    }
</script>
