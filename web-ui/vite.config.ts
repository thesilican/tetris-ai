import { defineConfig } from "vite";
import process from "node:process";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

export default defineConfig({
  base: process.env.BASE_URL ?? "/",
  plugins: [wasm(), topLevelAwait()],
  worker: {
    plugins: () => [wasm(), topLevelAwait()],
  },
});
