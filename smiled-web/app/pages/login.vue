<template>
  <div class="min-h-screen flex">
    <!-- Left panel — branding (hidden on mobile) -->
    <div class="hidden lg:flex lg:w-1/2 bg-[#0F172A] relative overflow-hidden flex-col justify-between p-12">
      <!-- Subtle dot pattern -->
      <div class="absolute inset-0 opacity-[0.03]">
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="grid-login" width="60" height="60" patternUnits="userSpaceOnUse">
              <circle cx="30" cy="30" r="1.5" fill="white" />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid-login)" />
        </svg>
      </div>
      <!-- Gradient accents -->
      <div class="absolute top-0 right-0 w-96 h-96 bg-gradient-to-bl from-teal-500/20 via-teal-600/10 to-transparent rounded-full blur-3xl -translate-y-1/2 translate-x-1/2" />
      <div class="absolute bottom-0 left-0 w-72 h-72 bg-gradient-to-tr from-teal-500/10 to-transparent rounded-full blur-2xl translate-y-1/3 -translate-x-1/3" />

      <!-- Logo -->
      <div class="relative z-10 flex items-center gap-3">
        <img src="~/assets/img/logomini.png" alt="Smiled.IO" class="size-10 rounded-xl" />
        <span class="text-2xl font-bold text-white tracking-tight">Smiled.IO</span>
      </div>

      <!-- Tagline -->
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

      <!-- Copyright -->
      <p class="relative z-10 text-slate-500 text-sm">&copy; {{ new Date().getFullYear() }} Smiled.IO</p>
    </div>

    <!-- Right panel — form -->
    <div class="flex w-full lg:w-1/2 items-center justify-center px-6 py-12 bg-white">
      <div class="w-full max-w-sm">
        <Transition appear name="fade-up">
          <div>
            <!-- Mobile logo -->
            <div class="lg:hidden flex justify-center mb-8">
              <img src="~/assets/img/logomini.png" alt="Smiled.IO" class="size-12 rounded-xl" />
            </div>

            <!-- Header -->
            <div class="mb-8">
              <h1 class="text-2xl font-semibold text-foreground tracking-tight">
                Connexion
              </h1>
              <p class="text-sm text-muted-foreground mt-1.5">
                Accedez a votre espace clinique
              </p>
            </div>

            <!-- Form -->
            <form class="space-y-5" @submit.prevent="handleLogin">
              <!-- Email -->
              <div class="space-y-2">
                <Label for="email">Email</Label>
                <Input
                  id="email"
                  v-model="email"
                  type="email"
                  placeholder="nom@cabinet.fr"
                  required
                  :disabled="loading"
                />
              </div>

              <!-- Password -->
              <div class="space-y-2">
                <Label for="password">Mot de passe</Label>
                <div class="relative">
                  <Input
                    id="password"
                    v-model="password"
                    :type="showPassword ? 'text' : 'password'"
                    placeholder="••••••••"
                    required
                    :disabled="loading"
                    class="pr-10"
                  />
                  <button
                    type="button"
                    class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                    tabindex="-1"
                    :aria-label="showPassword ? 'Masquer le mot de passe' : 'Afficher le mot de passe'"
                    @click="showPassword = !showPassword"
                  >
                    <EyeOff v-if="showPassword" class="size-4" />
                    <Eye v-else class="size-4" />
                  </button>
                </div>
              </div>

              <!-- Remember me -->
              <div class="flex items-center gap-2">
                <Checkbox id="remember" v-model:checked="rememberMe" />
                <Label for="remember" class="text-sm font-normal cursor-pointer">
                  Se souvenir de moi
                </Label>
              </div>

              <!-- Error -->
              <Alert v-if="error" variant="destructive">
                <AlertCircle class="size-4" />
                <AlertDescription>{{ error }}</AlertDescription>
              </Alert>

              <!-- Submit -->
              <Button type="submit" class="w-full" :disabled="loading">
                <Loader2 v-if="loading" class="size-4 mr-2 animate-spin" />
                Se connecter
              </Button>
            </form>

            <!-- Forgot password link -->
            <div class="mt-6 text-center">
              <NuxtLink
                to="/forgot-password"
                class="text-sm text-muted-foreground hover:text-primary transition-colors"
              >
                Mot de passe oublie ?
              </NuxtLink>
            </div>

            <!-- Footer -->
            <p class="text-xs text-muted-foreground mt-10 text-center">
              Smiled.IO — Logiciel dentaire
            </p>
          </div>
        </Transition>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { Eye, EyeOff, Loader2, AlertCircle } from 'lucide-vue-next'

definePageMeta({ layout: false })

const authStore = useAuthStore()

const email = ref('')
const password = ref('')
const showPassword = ref(false)
const rememberMe = ref(false)
const loading = ref(false)
const error = ref<string | null>(null)

async function handleLogin() {
  error.value = null
  loading.value = true

  try {
    await authStore.login(email.value, password.value)
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Identifiants invalides'
  } finally {
    loading.value = false
  }
}
</script>

<style scoped>
.fade-up-enter-active {
  transition: all 0.4s cubic-bezier(0.16, 1, 0.3, 1);
}

.fade-up-enter-from {
  opacity: 0;
  transform: translateY(12px);
}

.fade-up-enter-to {
  opacity: 1;
  transform: translateY(0);
}
</style>
