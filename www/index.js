const { TestStruct } = wasm_bindgen;

async function run() {
    await wasm_bindgen('./pkg/wallpaper_evolution_bg.wasm');

    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const epochBtn = document.getElementById('epoch');
    epochBtn.disabled = true;

    const worker = new Worker("worker.js");
    worker.onmessage = function (e) {
        const { type, payload } = e.data;
        switch (type) {
            case "ready":
                const url = "/public/evil_jerma.jpg";
                worker.postMessage({ type: "init/url", payload: url });
                break;
            case "init/done":
                epochBtn.disabled = false;
                break;
            case "epoch/done":
                const circle = payload;
                if (circle) {
                    const scale_x = canvas.clientWidth / circle.imgx;
                    const scale_y = canvas.clientHeight / circle.imgy;
                    ctx.fillStyle = circle.color;
                    ctx.beginPath();
                    ctx.ellipse(
                        circle.center_x * scale_x,
                        circle.center_y * scale_y,
                        circle.radius * scale_x,
                        circle.radius * scale_y,
                        0,
                        0,
                        2 * Math.PI
                    );
                    ctx.fill();
                } else {
                    console.log("No circle found");
                }
                worker.postMessage({ type: "epoch", payload: { num_gens: 1 } });
                break;
            default:
                console.error(`action type '${type}' not recognized`);
                break;
        }
    }

    epochBtn.addEventListener("click", () => {
        console.log("starting epoch");
        worker.postMessage({ type: "epoch", payload: { num_gens: 1 } });
        //console.log(struct.try_epoch(100, 50));
        //struct.draw(ctx);
    });

}

run();