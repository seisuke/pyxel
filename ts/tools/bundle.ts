import { bundle } from "https://deno.land/x/emit@0.31.1/mod.ts";

const result = await bundle(new URL("../src/index.ts", import.meta.url));

await Deno.writeTextFile("./src/index.js", result.code);
console.log("✅ Bundled src/index.ts -> src/index.js");
