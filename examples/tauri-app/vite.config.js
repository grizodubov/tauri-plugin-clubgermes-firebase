<<<<<<< HEAD
import { defineConfig } from "vite";
import { svelte } from "@sveltejs/vite-plugin-svelte";
import { internalIpV4Sync } from 'internal-ip'
=======
import {defineConfig} from "vite";
import {svelte} from "@sveltejs/vite-plugin-svelte";
import {internalIpV4Sync} from 'internal-ip'
>>>>>>> 273fbb6 (second ;-) commit)

const mobile = !!/android|ios/.exec(process.env.TAURI_ENV_PLATFORM);

// https://vitejs.dev/config/
<<<<<<< HEAD
export default defineConfig({
  plugins: [svelte()],

  // Vite optons tailored for Tauri development and only applied in `tauri dev` or `tauri build`
  // prevent vite from obscuring rust errors
  clearScreen: false,
  // tauri expects a fixed port, fail if that port is not available
  server: {
    host: mobile ? "0.0.0.0" : false,
    port: 1420,
    strictPort: true,
    hmr: mobile ? {
      protocol: 'ws',
      host: internalIpV4Sync(),
      port: 1421
    } : undefined,
  },
})
=======
export default defineConfig(async () => {
    return {
        plugins: [svelte()],

        // Vite optons tailored for Tauri development and only applied in `tauri dev` or `tauri build`
        // prevent vite from obscuring rust errors
        clearScreen: false,
        // tauri expects a fixed port, fail if that port is not available
        server: {
            host: mobile ? "0.0.0.0" : false,
            port: 1420,
            strictPort: true,
            hmr: mobile ? {
                protocol: 'ws',
                host: internalIpV4Sync(),
                port: 1421
            } : undefined,
        },
    }
});
>>>>>>> 273fbb6 (second ;-) commit)
