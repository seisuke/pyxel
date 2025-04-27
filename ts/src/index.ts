let wasmMemory: WebAssembly.Memory | undefined;

export async function run() {
  const wasmResponse = await fetch("./pyxel_wrapper_ts.wasm");
  const wasmArrayBuffer = await wasmResponse.arrayBuffer();

  const importObject = {
    env: {
      console_log: (ptr: number, len: number) => {
        if (!wasmMemory) {
          console.error("Memory not initialized!");
          return;
        }
        console.log("[Rust]", readString(wasmMemory, ptr, len));
      },
      memory: new WebAssembly.Memory({ initial: 256, maximum: 512 }), // 仮置き
    },
  };

  const { instance } = await WebAssembly.instantiate(wasmArrayBuffer, importObject);

  // メモリをちゃんとセットする
  wasmMemory = instance.exports.memory as WebAssembly.Memory;

  const exports = instance.exports as any;

  exports.init(160, 120);
  exports.update();
  exports.draw();
}

/**
 * メモリ上のptrからlenバイト読み出してUTF-8デコードする
 */
function readString(memory: WebAssembly.Memory, ptr: number, len: number): string {
  const memBuffer = new Uint8Array(memory.buffer, ptr, len);
  const decoder = new TextDecoder("utf-8");
  return decoder.decode(memBuffer);
}
