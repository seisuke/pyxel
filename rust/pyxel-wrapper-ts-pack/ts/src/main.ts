async function main() {
    const wasmData = await Deno.readFile("./pkg/pyxel_wrapper_ts.wasm");
    const { instance } = await WebAssembly.instantiate(wasmData, {
        env: {
            console_log: (ptr: number, len: number) => {
                const bytes = new Uint8Array(instance.exports.memory.buffer, ptr, len);
                const text = new TextDecoder("utf-8").decode(bytes);
                console.log("[Rust log]", text);
            }
        }
    });

    const exports = instance.exports as any;

    console.log("[Info] Instantiated WebAssembly module.");

    console.log("[Info1] Now calling Rust `init`...");
    exports.init(256, 256);

    console.log("[Info2] Now calling Rust `update`...");
    exports.update();

    console.log("[Info3] Now calling Rust `draw`...");
    exports.draw();
}

main();
