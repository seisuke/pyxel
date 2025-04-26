import { defineConfig } from "vite";

export default defineConfig({
    build: { target: "es2022" },
    publicDir: false               // pkg/ に Wasm が置かれるだけで OK
});
