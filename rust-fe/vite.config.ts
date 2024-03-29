import { defineConfig } from "vite";
import react from "@vitejs/plugin-react-swc";

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
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
