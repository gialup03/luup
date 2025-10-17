import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  
  // Tauri expects a fixed port in development
  server: {
    port: 5173,
    strictPort: true,
  },
  
  // Tauri uses a different pathname prefix in production
  build: {
    target: 'esnext',
  },
  
  // Prevent vite from obscuring rust errors
  clearScreen: false,
})
