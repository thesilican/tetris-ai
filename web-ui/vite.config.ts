import { defineConfig } from "vite";
import process from "node:process";

export default defineConfig({
  base: process.env.BASE_URL ?? "/",
});
