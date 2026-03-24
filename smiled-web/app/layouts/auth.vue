<template>
  <div class="min-h-screen flex flex-col lg:flex-row">
    <!-- Left panel: dark brand showcase -->
    <div class="hidden lg:flex lg:w-1/2 bg-[#0F172A] relative overflow-hidden flex-col justify-between p-12">
      <!-- Subtle pattern overlay -->
      <div class="absolute inset-0 opacity-[0.03]">
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="dental-grid" width="60" height="60" patternUnits="userSpaceOnUse">
              <circle cx="30" cy="30" r="1.5" fill="white" />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#dental-grid)" />
        </svg>
      </div>

      <!-- Gradient accent -->
      <div class="absolute top-0 right-0 w-96 h-96 bg-gradient-to-bl from-teal-500/20 via-teal-600/10 to-transparent rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" />
      <div class="absolute bottom-0 left-0 w-72 h-72 bg-gradient-to-tr from-teal-500/10 via-transparent to-transparent rounded-full blur-2xl translate-y-1/3 -translate-x-1/3" />

      <!-- Brand content -->
      <div class="relative z-10">
        <div class="flex items-center gap-3 mb-2">
          <div class="w-10 h-10 rounded-xl bg-gradient-to-br from-teal-400 to-teal-600 flex items-center justify-center">
            <Icon name="lucide:smile" class="w-6 h-6 text-white" />
          </div>
          <span class="text-2xl font-bold text-white tracking-tight">Smiled.IO</span>
        </div>
      </div>

      <!-- Tagline centered -->
      <div class="relative z-10 flex-1 flex flex-col justify-center max-w-md">
        <h2 class="text-3xl font-semibold text-white leading-tight mb-4">
          Logiciel dentaire
          <br />
          <span class="bg-gradient-to-r from-teal-400 to-teal-300 bg-clip-text text-transparent">
            nouvelle generation
          </span>
        </h2>
        <p class="text-slate-400 text-base leading-relaxed">
          Gerez vos patients, diagnostics et plans de traitement avec un outil moderne et intuitif.
        </p>
      </div>

      <!-- Footer -->
      <div class="relative z-10">
        <p class="text-slate-500 text-sm">&copy; {{ new Date().getFullYear() }} Smiled.IO</p>
      </div>
    </div>

    <!-- Right panel: form area -->
    <div class="flex-1 flex items-center justify-center bg-white p-6 sm:p-8 lg:p-12">
      <div class="w-full max-w-[420px]">
        <!-- Mobile brand header -->
        <div class="flex items-center gap-2 mb-8 lg:hidden">
          <div class="w-8 h-8 rounded-lg bg-gradient-to-br from-teal-400 to-teal-600 flex items-center justify-center">
            <Icon name="lucide:smile" class="w-5 h-5 text-white" />
          </div>
          <span class="text-xl font-bold text-foreground tracking-tight">Smiled.IO</span>
        </div>

        <slot />

        <!-- Footer on right panel -->
        <p class="text-xs text-muted-foreground mt-10 text-center">
          Smiled.IO — Logiciel dentaire
        </p>
      </div>
    </div>
  </div>
</template>
