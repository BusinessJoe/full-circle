// The worker has its own scope and no direct access to functions/objects of the
// global scope. We import the generated JS file to make `wasm_bindgen`
// available which we need to initialize our WASM code.
importScripts("./pkg/wallpaper_evolution.js");

console.log("Initializing worker")

// In the worker, we have a different struct that we want to use as in
// `index.js`.
//const {NumberEval} = wasm_bindgen;
const { TestStruct, JsRandomCircle } = wasm_bindgen;


let test_struct;

async function init_wasm_in_worker() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    await wasm_bindgen("./pkg/wallpaper_evolution_bg.wasm");

    await TestStruct.new_async("/public/evil_jerma.jpg")
        .then(struct => {
            console.log("loaded async!");
            test_struct = struct;

            // Set callback to handle messages passed to the worker.
            self.onmessage = async event => {
                console.log(test_struct, event.data);
                let best_circle = test_struct.try_epoch(100, event.data);
                console.log("circle before", best_circle, best_circle.imgx)

                // Send response back to be handled by callback in main thread.
                self.postMessage({
                    imgx: best_circle.imgx,
                    imgy: best_circle.imgy,
                    center_x: best_circle.center_x,
                    center_y: best_circle.center_y,
                    radius: best_circle.radius,
                    color: best_circle.color,
                });
            };

        });
};

// Create a new object of the `NumberEval` struct.
//var num_eval = NumberEval.new();


init_wasm_in_worker();