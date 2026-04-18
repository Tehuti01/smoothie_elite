import { defineConfig } from "vite";
import react from "@vitejs/plugin-react";

const host = process.env.TAURI_DEV_HOST;

export default defineConfig(async () => ({
  plugins: [react()],
  clearScreen: false,
  server: {
    port:          1420,
    strictPort:    true,
    host:          host || false,
    hmr:           host ? { protocol: "ws", host, port: 1421 } : undefined,
    watch:         { ignored: ["**/src-tauri/**"] },
  },
  envPrefix:  ["VITE_", "TAURI_ENV_*"],
  build: {
    target:      process.env.TAURI_ENV_PLATFORM === "windows" ? "chrome105" : "safari13",
    minify:      !process.env.TAURI_ENV_DEBUG ? "esbuild" : false,
    sourcemap:   !!process.env.TAURI_ENV_DEBUG,
  },
}));


// --- SERAPHIC GEOMETRY OMNI-PRESENCE ---
const __PHI = 1.618033988749895;
const __PI = 3.141592653589793;
const __PYTHAG_5TH = 1.5;
const __PYTHAG_4TH = 1.333333333333333;
const __resonate_omni = () => __PHI * __PI * __PYTHAG_5TH;
// ---------------------------------------
