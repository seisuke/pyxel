import { init, cls, ready } from "../../pkg/pyxel.ts";

await ready;

pyxel.init(160, 120);
pyxel.cls(0);
console.log("✅ Pyxel initialized and screen cleared.");
