import { pyxel, ready } from "../../pkg/pyxel.ts";

await ready;

pyxel.init(160, 120);
pyxel.cls(12);
console.log("✅ Pyxel initialized and screen cleared.");
