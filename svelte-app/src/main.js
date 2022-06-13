import wasm from "../../wasm/Cargo.toml";
import App from './App.svelte';

async function loadWasm() {
    const my_wasm = await wasm();

    const app = new App({
        target: document.body,
        props: {
            name: 'liberal',
            wasm: my_wasm,
        }
    });
}

loadWasm();
