import { defineConfig } from "vite";
import fixReactVirtualized from "esbuild-plugin-react-virtualized";
import path from "path";
import react from "@vitejs/plugin-react";

// https://vitejs.dev/config/
export default defineConfig(async () => ({
  plugins: [
    react(),
    {
      name: "markdown-loader",
      transform(code, id) {
        if (id.slice(-3) === ".md") {
          // For .md files, get the raw content
          return `export default ${JSON.stringify(code)};`;
        }
      },
    },
  ],

  optimizeDeps: {
    esbuildOptions: {
      plugins: [fixReactVirtualized],
    },
  },

  resolve: {
    alias: {
      "@": path.resolve(__dirname, "./crates/ui/src"),
    },
  },

  // Vite options tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  //
  // 1. prevent vite from obscuring rust errors
  clearScreen: false,
  // 2. tauri expects a fixed port, fail if that port is not available
  server: {
    port: 1420,
    strictPort: true,
    watch: {
      // 3. tell vite to ignore watching `src-tauri`
      // cwd: path.resolve(__dirname, "./crates/"),
      ignored: ["crates/**"],
    },
  },
}));
