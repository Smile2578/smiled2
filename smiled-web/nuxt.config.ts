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
    'shadcn-nuxt',
    '@nuxtjs/color-mode',
  ],

  // Force light mode (no dark mode implemented yet)
  colorMode: {
    preference: 'light',
    fallback: 'light',
    classSuffix: '',
    storage: 'cookie',
    storageKey: 'smiled-color-mode',
  },

  // shadcn-vue: module handles ui/ component scanning without conflicts
  shadcn: {
    prefix: '',
    componentDir: './app/components/ui',
  },

  // Prevent Nuxt default scan from conflicting with shadcn-nuxt module
  ignore: ['app/components/ui/**'],

  // Tailwind CSS configuration
  tailwindcss: {
    cssPath: '~/assets/css/tailwind.css',
    configPath: 'tailwind.config.ts',
  },

  // Runtime config — public vars exposed to the browser
  runtimeConfig: {
    public: {
      apiBase: process.env.NUXT_PUBLIC_API_BASE ?? 'http://localhost:8080',
    },
  },

  // TypeScript
  typescript: {
    strict: true,
    typeCheck: false, // Enable manually with `npm run typecheck`
  },

  // Fonts — Inter with all needed weights
  fonts: {
    families: [
      {
        name: 'Inter',
        provider: 'google',
        weights: [400, 500, 600, 700],
      },
    ],
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
    optimizeDeps: {
      include: [
        'lucide-vue-next',
        '@vueuse/core',
        'reka-ui',
        'class-variance-authority',
        'clsx',
        'tailwind-merge',
        'vue-sonner',
        '@vue/devtools-core',
        '@vue/devtools-kit',
      ],
    },
  },
})
