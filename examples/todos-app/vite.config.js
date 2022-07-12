import { sveltekit } from '@sveltejs/kit/vite';

/** @type {import('vite').UserConfig} */
const config = {
  plugins: [sveltekit()],
  build: {
    rollupOptions: {
      external: ['@tauri-apps/api/tauri']
    }
  }
};

export default config;