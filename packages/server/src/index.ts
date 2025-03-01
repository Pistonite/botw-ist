// const f = Bun.file("app/index.html");
// console.log(await f.text());

import { createAppRoutes } from "app";
import { routeBuilder, useLogging } from "framework";

const hostname = "0.0.0.0";
const port = 8000;
console.log("starting server on http://" + hostname + ":" + port);

const builder = routeBuilder().inbound(useLogging);


Bun.serve({
    port,
    reusePort: true,
    hostname,
    routes: {
        ...(await createAppRoutes(builder)),
        // "/-/*": (req) => {
        //     const pathname = new URL(req.url).pathname;
        //     if (!pathname.startsWith("/-/")) {
        //         return new Response("Not Found", { status: 404 });
        //     }
        //     const directURL = pathname.replace(/^\/-\//, "https://");
        //
        //     console.log(directURL);
        //     return new Response(Bun.file("app/index.html"))
        // },
    },
    // Global error handler
  error(error) {
    console.error(error);
    return new Response(`Internal Error: ${error.message}`, {
      status: 500,
      headers: {
        "Content-Type": "text/plain",
      },
    });
  },
});
