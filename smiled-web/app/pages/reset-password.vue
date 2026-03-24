<template>
  <div class="min-h-screen flex">
    <!-- Left panel — branding (hidden on mobile) -->
    <div class="hidden lg:flex lg:w-1/2 bg-[#0F172A] relative overflow-hidden flex-col justify-between p-12">
      <!-- Subtle dot pattern -->
      <div class="absolute inset-0 opacity-[0.03]">
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="grid-reset" width="60" height="60" patternUnits="userSpaceOnUse">
              <circle cx="30" cy="30" r="1.5" fill="white" />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid-reset)" />
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
                Nouveau mot de passe
              </h1>
              <p class="text-sm text-muted-foreground mt-1.5">
                Choisissez un nouveau mot de passe pour votre compte
              </p>
            </div>

            <!-- Invalid token -->
            <div v-if="!token" class="space-y-6">
              <Alert variant="destructive">
                <AlertCircle class="size-4" />
                <AlertDescription>
                  Lien invalide ou expire. Veuillez refaire une demande de reinitialisation.
                </AlertDescription>
              </Alert>
              <Button variant="outline" class="w-full" @click="navigateTo('/forgot-password')">
                Nouvelle demande
              </Button>
            </div>

            <!-- Success state -->
            <div v-else-if="success" class="space-y-6">
              <div class="flex flex-col items-center text-center py-4">
                <div class="size-12 rounded-full bg-teal-50 flex items-center justify-center mb-4">
                  <CheckCircle2 class="size-6 text-teal-600" />
                </div>
                <p class="text-sm font-medium text-foreground mb-1">Mot de passe reinitialise</p>
                <p class="text-sm text-muted-foreground">
                  Votre mot de passe a ete reinitialise avec succes.
                </p>
              </div>

              <Button class="w-full" @click="navigateTo('/login')">
                Se connecter
              </Button>
            </div>

            <!-- Form -->
            <form v-else class="space-y-5" @submit.prevent="handleSubmit">
              <!-- New password -->
              <div class="space-y-2">
                <Label for="password">Nouveau mot de passe</Label>
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

                <!-- Password strength indicator -->
                <div class="space-y-1.5">
                  <div class="flex gap-1">
                    <div
                      v-for="i in 4"
                      :key="i"
                      class="h-1 flex-1 rounded-full transition-colors duration-200"
                      :class="i <= passwordStrength.level ? strengthColors[passwordStrength.level] : 'bg-muted'"
                    />
                  </div>
                  <p v-if="password" class="text-xs" :class="strengthTextColors[passwordStrength.level]">
                    {{ passwordStrength.label }}
                  </p>
                  <p v-else class="text-xs text-muted-foreground">
                    Minimum 12 caracteres
                  </p>
                </div>
              </div>

              <!-- Confirm password -->
              <div class="space-y-2">
                <Label for="confirm">Confirmer le mot de passe</Label>
                <div class="relative">
                  <Input
                    id="confirm"
                    v-model="confirm"
                    :type="showConfirm ? 'text' : 'password'"
                    placeholder="••••••••"
                    required
                    :disabled="loading"
                    class="pr-10"
                  />
                  <button
                    type="button"
                    class="absolute right-3 top-1/2 -translate-y-1/2 text-muted-foreground hover:text-foreground transition-colors"
                    tabindex="-1"
                    :aria-label="showConfirm ? 'Masquer le mot de passe' : 'Afficher le mot de passe'"
                    @click="showConfirm = !showConfirm"
                  >
                    <EyeOff v-if="showConfirm" class="size-4" />
                    <Eye v-else class="size-4" />
                  </button>
                </div>
              </div>

              <!-- Error -->
              <Alert v-if="error" variant="destructive">
                <AlertCircle class="size-4" />
                <AlertDescription>{{ error }}</AlertDescription>
              </Alert>

              <!-- Submit -->
              <Button type="submit" class="w-full" :disabled="loading">
                <Loader2 v-if="loading" class="size-4 mr-2 animate-spin" />
                Reinitialiser
              </Button>
            </form>

            <!-- Back link -->
            <div class="mt-6 text-center">
              <NuxtLink
                to="/login"
                class="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-primary transition-colors"
              >
                <ArrowLeft class="size-3.5" />
                Retour a la connexion
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
import { Eye, EyeOff, Loader2, ArrowLeft, CheckCircle2, AlertCircle } from 'lucide-vue-next'

definePageMeta({ layout: false })

const authClient = useAuthClient()
const route = useRoute()

const token = computed(() => route.query.token as string | undefined)

const password = ref('')
const confirm = ref('')
const showPassword = ref(false)
const showConfirm = ref(false)
const loading = ref(false)
const error = ref<string | null>(null)
const success = ref(false)

const strengthColors: Record<number, string> = {
  0: 'bg-muted',
  1: 'bg-red-500',
  2: 'bg-orange-500',
  3: 'bg-yellow-500',
  4: 'bg-teal-500',
}

const strengthTextColors: Record<number, string> = {
  0: 'text-muted-foreground',
  1: 'text-red-500',
  2: 'text-orange-500',
  3: 'text-yellow-600',
  4: 'text-teal-600',
}

const passwordStrength = computed(() => {
  const val = password.value
  if (!val) return { level: 0, label: '' }

  let score = 0
  if (val.length >= 12) score++
  if (/[A-Z]/.test(val) && /[a-z]/.test(val)) score++
  if (/\d/.test(val)) score++
  if (/[^A-Za-z0-9]/.test(val)) score++

  const labels: Record<number, string> = {
    0: 'Tres faible',
    1: 'Faible',
    2: 'Moyen',
    3: 'Fort',
    4: 'Tres fort',
  }

  return { level: score, label: labels[score] }
})

async function handleSubmit() {
  error.value = null

  if (password.value !== confirm.value) {
    error.value = 'Les mots de passe ne correspondent pas'
    return
  }

  if (password.value.length < 12) {
    error.value = 'Le mot de passe doit contenir au moins 12 caracteres'
    return
  }

  loading.value = true

  try {
    const result = await authClient.resetPassword({
      newPassword: password.value,
      token: token.value!,
    })
    if (result.error) {
      throw new Error(result.error.message || 'Une erreur est survenue')
    }
    success.value = true
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Une erreur est survenue'
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
