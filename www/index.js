import * as wasm from "wasm-full-circle";
import { TestStruct } from "wasm-full-circle";

const canvas = document.getElementById('drawing');
const ctx = canvas.getContext('2d');

const epochBtn = document.getElementById('epoch');

const struct = new TestStruct("test.png");
struct.draw(ctx);


epochBtn.addEventListener("click", () => {
    console.log(struct.try_epoch(50, 10));
    struct.draw(ctx);
});
