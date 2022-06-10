const room_id = location.pathname.split('/')[2];

const uri = 'ws://' + location.hostname + ':3001' + '/join/' + room_id;
console.log(uri);
const ws = new WebSocket(uri);
ws.onopen = () => {console.log("Websocket opened");};
ws.onmessage = (message) => drawCircle(JSON.parse(message.data));

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

async function run() {
    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const epochBtn = document.getElementById('epoch');
}

window.addEventListener("load", async function() {
    console.log("loaded");
    await run();
});
