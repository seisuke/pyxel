import { pyxel } from "../pkg/pyxel.ts"

await pyxel.init(640, 360)
await pyxel.load("sample.pyxres")
pyxel.cls(12)
let a = new pyxel.Image(100, 100)

console.log("✅ Pyxel initialized and screen cleared.")
