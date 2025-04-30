import * as esbuild from "https://deno.land/x/esbuild@v0.19.11/mod.js";

const result = await esbuild.build({
  entryPoints: ["./ts/src/index.ts"],
  bundle: true,
  format: "esm",
  write: false,
  platform: "browser",
});

await Deno.writeTextFile("./pkg/index.js", result.outputFiles[0].text);
console.log("✅ Bundled ts/src/index.ts → pkg/index.js");

esbuild.stop();
