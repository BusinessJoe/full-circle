const { TestStruct } = wasm_bindgen;

async function run() {
    await wasm_bindgen('./pkg/wallpaper_evolution_bg.wasm');

    console.log("among");

    const canvas = document.getElementById('drawing');
    const ctx = canvas.getContext('2d');

    const epochBtn = document.getElementById('epoch');

    const worker = new Worker("worker.js");
    worker.onmessage = function (e) {
        const circle = e.data;
        console.log("circle", circle, circle.imgx);
        if (circle) {
            const scale_x = canvas.clientWidth / circle.imgx;
            const scale_y = canvas.clientHeight / circle.imgy;
            ctx.fillStyle = circle.color;
            ctx.beginPath();
            console.log(circle.imgx, circle.imgy);
            console.log(circle.center_x * scale_x,
                circle.center_y * scale_y,
                circle.radius * scale_x,
                circle.radius * scale_y);
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
    }

    epochBtn.addEventListener("click", () => {
        console.log("starting epoch");
        worker.postMessage(25);
        //console.log(struct.try_epoch(100, 50));
        //struct.draw(ctx);
    });
}

run();