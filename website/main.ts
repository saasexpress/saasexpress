import { serveDir, serveFile } from "jsr:@std/http/file-server";

Deno.serve((req: Request) => {
    return serveDir(req, {
      fsRoot: "site"
    });

});