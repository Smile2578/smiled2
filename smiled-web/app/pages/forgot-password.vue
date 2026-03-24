<template>
  <div class="min-h-screen flex">
    <!-- Left panel — branding (hidden on mobile) -->
    <div class="hidden lg:flex lg:w-1/2 bg-[#0F172A] relative overflow-hidden flex-col justify-between p-12">
      <!-- Subtle dot pattern -->
      <div class="absolute inset-0 opacity-[0.03]">
        <svg class="w-full h-full" xmlns="http://www.w3.org/2000/svg">
          <defs>
            <pattern id="grid-forgot" width="60" height="60" patternUnits="userSpaceOnUse">
              <circle cx="30" cy="30" r="1.5" fill="white" />
            </pattern>
          </defs>
          <rect width="100%" height="100%" fill="url(#grid-forgot)" />
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

            <!-- Back link at top -->
            <NuxtLink
              to="/login"
              class="inline-flex items-center gap-1.5 text-sm text-muted-foreground hover:text-primary transition-colors mb-8"
            >
              <ArrowLeft class="size-3.5" />
              Retour a la connexion
            </NuxtLink>

            <!-- Header -->
            <div class="mb-8">
              <h1 class="text-2xl font-semibold text-foreground tracking-tight">
                Mot de passe oublie
              </h1>
              <p class="text-sm text-muted-foreground mt-1.5">
                Entrez votre email pour recevoir un lien de reinitialisation
              </p>
            </div>

            <!-- Success state -->
            <div v-if="sent" class="space-y-6">
              <div class="flex flex-col items-center text-center py-4">
                <div class="size-12 rounded-full bg-teal-50 flex items-center justify-center mb-4">
                  <CheckCircle2 class="size-6 text-teal-600" />
                </div>
                <p class="text-sm font-medium text-foreground mb-1">Email envoye</p>
                <p class="text-sm text-muted-foreground">
                  Si un compte existe pour cette adresse, un email de reinitialisation vous a ete envoye.
                </p>
              </div>

              <Button variant="outline" class="w-full" @click="resetForm">
                Envoyer a nouveau
              </Button>
            </div>

            <!-- Form -->
            <form v-else class="space-y-5" @submit.prevent="handleSubmit">
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

              <!-- Error -->
              <Alert v-if="error" variant="destructive">
                <AlertCircle class="size-4" />
                <AlertDescription>{{ error }}</AlertDescription>
              </Alert>

              <!-- Submit -->
              <Button type="submit" class="w-full" :disabled="loading">
                <Loader2 v-if="loading" class="size-4 mr-2 animate-spin" />
                Envoyer le lien
              </Button>
            </form>

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
import { Loader2, ArrowLeft, CheckCircle2, AlertCircle } from 'lucide-vue-next'

definePageMeta({ layout: false })

const authClient = useAuthClient()

const email = ref('')
const loading = ref(false)
const error = ref<string | null>(null)
const sent = ref(false)

async function handleSubmit() {
  error.value = null
  loading.value = true

  try {
    const result = await authClient.requestPasswordReset({
      email: email.value,
      redirectTo: '/reset-password',
    })
    if (result.error) {
      throw new Error(result.error.message || 'Une erreur est survenue')
    }
    sent.value = true
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Une erreur est survenue'
  } finally {
    loading.value = false
  }
}

function resetForm() {
  sent.value = false
  email.value = ''
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
