import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";
import wasm from "vite-plugin-wasm";
import topLevelAwait from "vite-plugin-top-level-await";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react(), wasm(), topLevelAwait()],
  optimizeDeps: {
    exclude: ["rust-wasm"],
  },
  server: {
    proxy: {
      // 使用 proxy 实例
      "/api": {
        target: "http://127.0.0.1:3001",
        changeOrigin: true,
        configure: () => {},
      },
      "/ws": {
        target: "ws://127.0.0.1:3001",
        changeOrigin: true,
        configure: () => {},
      },
    },
  },
});
