import { serve } from "https://deno.land/std@0.224.0/http/server.ts";
import { serveFile } from "https://deno.land/std@0.224.0/http/file_server.ts";
import * as esbuild from "https://deno.land/x/esbuild@v0.19.11/mod.js";

console.log("📡 Serving at http://localhost:8000/");

serve(async (req) => {
  const url = new URL(req.url);
  const pathname = url.pathname === "/" ? "/index.html" : url.pathname;
  const filepath = `./pkg${pathname}`;

  if (pathname === "/index.js") {
    const result = await esbuild.build({
      entryPoints: ["ts/src/index.ts"],
      bundle: true,
      write: false,
      format: "esm",
      platform: "browser",
    });

    return new Response(result.outputFiles[0].text, {
      headers: { "content-type": "application/javascript" },
    });
  }

  try {
    return await serveFile(req, filepath);
  } catch {
    return new Response("404 Not Found", { status: 404 });
  }
}, { port: 8000 });

// サーバ終了時にesbuildも終了
addEventListener("unload", () => {
  esbuild.stop();
});
