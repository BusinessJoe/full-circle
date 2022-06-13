<script context="module">
    // onEvent(type, payload)
    export function initWebsocket(uri, onEvent) {
        const canvas = document.getElementById('drawing');

        console.log(uri);
        const ws = new WebSocket(uri);

        ws.onopen = () => {console.log("Websocket opened");};
        ws.onmessage = (message) => {
            console.log('received from ws:', JSON.parse(message.data));
            const data = JSON.parse(message.data);
            const type = Object.keys(data)[0];
            const payload = data[type];
            onEvent(type, payload);
        };

        return ws;
    }


    // Helper function for sending an event through a websocket
    export function sendWsEvent(ws, type, payload) {
        const message = JSON.stringify({ [type]: payload });
        ws.send(message);
    }
</script>
