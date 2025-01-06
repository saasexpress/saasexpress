// src/mocks/handlers.js
import { http, HttpResponse, delay, passthrough } from "msw";

import { ShellItemProps } from "../shell/item.tsx";
import ProxyRequest from "./proxy.ts";

export const handlers = [
  http.all("https://use.fontawesome.com/releases/v5.3.1/css/all.css", () => {
    return passthrough();
  }),
  http.all("https://fonts.gstatic.com/**", () => {
    return passthrough();
  }),
  http.all("https://cdn.jsdelivr.net/npm/monaco-editor@0.43.0/**", () => {
    return passthrough();
  }),
  http.all("/ui/**", () => {
    return passthrough();
  }),
  http.get("/api/runtime", async () => {
    return HttpResponse.json({});
  }),

  http.post("/api/tenants", async ({ request }) => {
    return ProxyRequest({ request });
  }),

  http.get("/api/tenants", async () => {
    await delay(500);
    return HttpResponse.json([
      {
        _id: "1",
        status: "active",
        label: "A Shell for more",
        description: "John",
        building_blocks: [],
      },
    ] as ShellItemProps[]);
  }),
];
