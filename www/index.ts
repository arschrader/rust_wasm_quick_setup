import init, { World } from "rust_wasm_quick_setup";

init().then(_ => {
    const world = World.new();
    console.log(world.width);
})