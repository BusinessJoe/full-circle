const { TestStruct } = wasm_bindgen;

let worker;

const uri = 'ws://' + location.hostname + ':3001' + '/room';
console.log(uri);
const ws = new WebSocket(uri);
ws.onopen = () => {console.log("Websocket opened");};
ws.onmessage = (message) => drawCircle(JSON.parse(message.data));

function readSingleFile(e) {
    let file = e.target.files[0];
    if (!file) {
        return;
    }

    file.arrayBuffer().then(buffer => {
        worker.postMessage({ type: "init/buffer", payload: buffer });
    });
}

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
    await wasm_bindgen('js/pkg/wasm_bg.wasm');

    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const epochBtn = document.getElementById('epoch');
    //epochBtn.disabled = true;

    document.getElementById('file-input')
        .addEventListener('change', readSingleFile, false);

    worker = new Worker("js/worker.js");
    worker.onmessage = function (e) {
        const { type, payload } = e.data;
        console.log("processing event", type);
        switch (type) {
            case "ready":
                const url = "/images/moon.jpeg";
                worker.postMessage({ type: "init/url", payload: url });
                break;
            case "init/done":
                const [width, height] = payload;
                console.log(width, height);
                canvas.width = width;
                canvas.height = height;
                epochBtn.disabled = false;
                break;
            case "epoch/done":
                if (payload) {
                    const { circle, image_data } = payload;
                    // Turn typed arrays (Int32Array, etc.) into normal JS arrays.
                    circle.center = Array.from(circle.center);
                    circle.color = Array.from(circle.color);

                    const message = JSON.stringify(circle);
                    console.log(circle);
                    console.log(message);
                    ws.send(message);
                } else {
                    console.log("No circle found");
                }
                worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
                break;
            default:
                console.error(`action type '${type}' not recognized`);
                break;
        }
    }

    epochBtn.addEventListener("click", () => {
        console.log("starting epoch");
        worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
        //console.log(struct.try_epoch(100, 50));
        //struct.draw(ctx);
    });

}

run();
