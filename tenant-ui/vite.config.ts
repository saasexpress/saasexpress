import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";
import react from "@vitejs/plugin-react";
import tailwindcss from "@tailwindcss/vite";

// https://vite.dev/config/
export default defineConfig({
  // root: ".",
  base: "/ui",
  plugins: [tailwindcss() as any, react(), tsconfigPaths()],
  // build: {
  //   // outDir: "dist",
  //   // lib: {
  //   //   entry: "app/root.tsx",
  //   //   name: "MyLib",
  //   //   fileName: (format) => `my-lib.${format}.js`,
  //   // },
  // },
  // // optimizeDeps: {
  // //   include: ["@emotion/styled"],
  // // },
});
