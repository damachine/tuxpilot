import { sveltekit } from '@sveltejs/kit/vite';
import { defineConfig } from 'vite';

export default defineConfig({
	plugins: [sveltekit()],
	server: {
		proxy: {
			'/api': {
				target: 'http://127.0.0.1:8082',
				changeOrigin: true
			},
			'/ws': {
				target: 'ws://127.0.0.1:8082',
				ws: true
			}
		}
	}
});
