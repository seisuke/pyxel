import { pyxel } from "../pkg/pyxel.ts"

await pyxel.init(640, 360)
await pyxel.load("sample.pyxres")
console.log(pyxel.images.len())
pyxel.cls(12)
console.log("✅ Pyxel initialized and screen cleared.")
