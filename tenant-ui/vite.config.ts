import { defineConfig } from "vite";
import tsconfigPaths from "vite-tsconfig-paths";
import react from "@vitejs/plugin-react";

// https://vite.dev/config/
export default defineConfig({
  // root: ".",
  base: "/ui",
  plugins: [react(), tsconfigPaths()],
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
