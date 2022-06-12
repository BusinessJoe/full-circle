
function drawCircle(circle) {
    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const { imgx, imgy, center, radius, color } = circle;

    const scale_x = canvas.width / imgx;
    const scale_y = canvas.height / imgy;
    ctx.fillStyle = `rgba(${color[0]},${color[1]},${color[2]},${color[3]/255}`;
    ctx.beginPath();
    ctx.ellipse(
        center[0] * scale_x,
        center[1] * scale_y,
        radius * scale_x,
        radius * scale_y,
        0,
        0,
        2 * Math.PI
    );
    ctx.fill();
}

function initWebsocket(uri) {
    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');
    const epochBtn = document.getElementById('epoch');

    console.log(uri);
    const ws = new WebSocket(uri);

    ws.onopen = () => {console.log("Websocket opened");};
    ws.onmessage = (message) => {
        console.log(JSON.parse(message.data));
        const data = JSON.parse(message.data);
        const type = Object.keys(data)[0];
        const payload = data[type];
        switch (type) {
            case "Circle":
                drawCircle(payload);
                break;
            case "RoomPath":
                const link_elem = document.getElementById("room-link");
                const link_text_elem = document.getElementById("room-link-text");

                const link = location.host + payload;

                link_elem.href = link;
                link_text_elem.innerHTML = link;
                break;
            case "NewImage":
                const [width, height] = payload.dimensions;
                canvas.width = width;
                canvas.height = height;
                break;
        }
        console.log('received', message.data);
    };

    return ws;
}

function sendWsEvent(ws, type, payload) {
    const message = JSON.stringify({ [type]: payload });
    ws.send(message);
}

module.exports = {
    drawCircle, 
    initWebsocket,
    sendWsEvent,
};
