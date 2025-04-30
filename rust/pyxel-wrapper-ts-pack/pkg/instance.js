export const instancePromise = WebAssembly.instantiateStreaming(
  fetch("/pyxel_wrapper_ts.wasm"),
  {
    env: {
      memory: new WebAssembly.Memory({ initial: 256, maximum: 512 }),
      // 必要であれば関数も追加
      // console_log: (ptr, len) => { ... }
    },
  }
).then((result) => result.instance);
