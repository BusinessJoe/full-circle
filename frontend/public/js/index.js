const { drawCircle, initWebsocket, sendWsEvent } = require('./common.js');
const { TestStruct } = wasm_bindgen;

function readSingleFile(e) {
    let file = e.target.files[0];
    if (!file) {
        return;
    }

    file.arrayBuffer().then(buffer => {
        worker.postMessage({ type: "init/buffer", payload: buffer });
    });
}

async function initWebWorker(ws) {
    const canvas = document.getElementById('drawing');
    const epochBtn = document.getElementById('epoch');

    await wasm_bindgen('js/pkg/wasm_bg.wasm');

    let worker = new Worker("js/worker.js");
    worker.onmessage = function (e) {
        const { type, payload } = e.data;
        console.log("processing worker event", type);
        switch (type) {
            case "ready":
                const url = "/images/moon.jpeg";
                worker.postMessage({ type: "init/url", payload: url });
                break;
            case "init/done":
                const [width, height] = payload;
                sendWsEvent(ws, "NewImage", { dimensions: [width, height] });
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

                    const message = JSON.stringify({"Circle": circle});
                    sendWsEvent(ws, "Circle", circle);
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

    return worker;
}

async function run() {
    await wasm_bindgen('js/pkg/wasm_bg.wasm');

    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const epochBtn = document.getElementById('epoch');

    const uri = 'ws://' + location.hostname + ':3001' + '/room';
    const ws = initWebsocket(uri);

    document.getElementById('file-input')
        .addEventListener('change', readSingleFile, false);

    let worker = await initWebWorker(ws);

    epochBtn.addEventListener("click", () => {
        console.log("starting epoch");
        worker.postMessage({ type: "epoch", payload: { num_gens: 25, gen_size: 100 } });
        //console.log(struct.try_epoch(100, 50));
        //struct.draw(ctx);
    });
}

run();
