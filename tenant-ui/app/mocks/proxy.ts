import { bypass } from "msw";

export default async function ProxyRequest({ request }: any) {
  //const originalUrl = new URL(request.url);
  const proxyUrl = new URL("http://localhost:8081", location.origin);

  // Construct a proxy request.
  const proxyRequest = new Request(proxyUrl, {
    headers: {
      "Content-Type": request.headers.get("Content-Type"),
      "X-Proxy-Header": "true",
    },
  });

  // Perform the proxy request.
  return await fetch(bypass(proxyRequest));
}
