// The worker has its own scope and no direct access to functions/objects of the
// global scope. We import the generated JS file to make `wasm_bindgen`
// available which we need to initialize our WASM code.
import wasm from "../../wasm/Cargo.toml";

console.log("Initializing worker")

// In the worker, we have a different struct that we want to use as in
// `index.js`.
//const {NumberEval} = wasm_bindgen;
//const { TestStruct, JsRandomCircle } = wasm_bindgen;


let test_struct;

async function init_wasm_in_worker() {
    // Load the wasm file by awaiting the Promise returned by `wasm_bindgen`.
    const module = await wasm();
    const { TestStruct } = module;

    // Set callback to handle messages passed to the worker.
    self.onmessage = async event => {
        const { type, payload } = event.data;

        switch (type) {
            case "init/url":
                const url = payload;
                await TestStruct.new_async(url)
                    .then(struct => {
                        console.log("loaded async!");
                        test_struct = struct;
                        self.postMessage({
                            type: "init/done",
                            payload: [test_struct.get_target_width(), test_struct.get_target_height()]
                        });
                    })
                    .catch(err => {
                        console.error(err);
                        self.postMessage({ type: "init/error" });
                    });
                break;
            case "init/buffer":
                const buffer = payload;
                test_struct = TestStruct.new_from_buffer(buffer);
                console.log("loaded from buffer!");
                self.postMessage({
                    type: "init/done",
                    payload: [test_struct.get_target_width(), test_struct.get_target_height()]
                });
                break;
            case "epoch":
                const { gen_size, num_gens } = payload;
                let best_circle = test_struct.try_epoch(gen_size, num_gens);

                let outbound_payload;
                if (best_circle) {
                    outbound_payload = { 
                        circle: {
                            imgx: best_circle.imgx,
                            imgy: best_circle.imgy,
                            center: best_circle.center,
                            radius: best_circle.radius,
                            color: best_circle.color,
                        }
                    };
                } else {
                    outbound_payload = undefined;
                }

                // Send response back to be handled by callback in main thread.
                self.postMessage({
                    type: "epoch/done",
                    payload: outbound_payload,
                });
                break;
            default:
                console.error(`action type '${type}' not recognized`);
                break;
        }
    };

    self.postMessage({ type: "ready" });
};

init_wasm_in_worker();
