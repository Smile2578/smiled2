// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  compatibilityDate: '2024-11-01',

  // Future-compatible flags for Nuxt 4
  future: {
    compatibilityVersion: 4,
  },

  // Modules
  modules: [
    '@nuxtjs/tailwindcss',
    '@pinia/nuxt',
    '@nuxt/icon',
    '@nuxt/fonts',
  ],

  // Tailwind CSS configuration
  tailwindcss: {
    cssPath: '~/assets/css/tailwind.css',
    configPath: 'tailwind.config.ts',
  },

  // Runtime config — public vars exposed to the browser
  runtimeConfig: {
    // Private (server-only)
    apiSecret: '',

    // Public (exposed to browser)
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE ?? 'http://localhost:8080',
    },
  },

  // TypeScript
  typescript: {
    strict: true,
    typeCheck: false, // Enable manually with `npm run typecheck`
  },

  // Dev tools
  devtools: { enabled: true },

  // Vite config
  vite: {
    server: {
      hmr: {
        protocol: 'ws',
      },
    },
  },
})
