<script context="module">
export class PlayerWebSocket extends EventTarget {
    constructor(uri, name) {
        super();
        this.ws = null;
        this.connectWebSocket(uri, name);
    }

    connectWebSocket(uri, name) {
        console.log("Establishing connection");
        this.ws = new WebSocket(uri);

        this.ws.onopen = () => {
            console.log("Websocket opened successfully");
            this.send("PlayerName", name);
        };

        this.ws.onmessage = (message) => {
            let type, payload;
            if (message.data instanceof Blob) {
                type = "binary";
                payload = message.data;
            } else {
                const data = JSON.parse(message.data);
                type = Object.keys(data)[0];
                payload = data[type];
            }

            let event = new CustomEvent(type, {
                detail: {
                    payload
                }
            });
            this.dispatchEvent(event);
        }
    }
    
    close() {
        this.ws.close();
    }

    send(type, payload) {
        if (payload === undefined) {
            // this is how unit structs are represented
            payload = null;
        }
        const message = JSON.stringify({ [type]: payload });
        this.ws.send(message);
    }

    addEventListener(type, listener) {
        console.log("Adding listener for", type);
        const decorated_listener = (event) => {
            listener(event.detail.payload);
        }
        super.addEventListener(type, decorated_listener);
    }
}
</script>
