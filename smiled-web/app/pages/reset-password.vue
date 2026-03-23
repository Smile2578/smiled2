<template>
  <NuxtLayout name="auth">
    <!-- Branding -->
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold text-primary">Smiled.IO</h1>
      <p class="text-muted-foreground mt-1">Nouveau mot de passe</p>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Réinitialiser le mot de passe</CardTitle>
        <CardDescription>
          Choisissez un nouveau mot de passe pour votre compte
        </CardDescription>
      </CardHeader>

      <CardContent>
        <Alert v-if="!token" variant="destructive">
          <AlertDescription>
            Lien invalide ou expiré. Veuillez refaire une demande de réinitialisation.
          </AlertDescription>
        </Alert>

        <div v-else-if="success" class="space-y-4">
          <Alert>
            <AlertDescription>
              Votre mot de passe a été réinitialisé avec succès.
            </AlertDescription>
          </Alert>
          <Button class="w-full" @click="navigateTo('/login')">
            Se connecter
          </Button>
        </div>

        <form v-else class="space-y-4" @submit.prevent="handleSubmit">
          <div class="space-y-2">
            <Label for="password">Nouveau mot de passe</Label>
            <Input
              id="password"
              v-model="password"
              type="password"
              placeholder="••••••••"
              required
              :disabled="loading"
            />
          </div>

          <div class="space-y-2">
            <Label for="confirm">Confirmer le mot de passe</Label>
            <Input
              id="confirm"
              v-model="confirm"
              type="password"
              placeholder="••••••••"
              required
              :disabled="loading"
            />
          </div>

          <Alert v-if="error" variant="destructive">
            <AlertDescription>{{ error }}</AlertDescription>
          </Alert>

          <Button type="submit" class="w-full" :disabled="loading">
            <Icon v-if="loading" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
            Réinitialiser le mot de passe
          </Button>
        </form>
      </CardContent>

      <CardFooter class="flex justify-center">
        <NuxtLink
          to="/login"
          class="text-sm text-muted-foreground hover:text-primary transition-colors"
        >
          Retour à la connexion
        </NuxtLink>
      </CardFooter>
    </Card>
  </NuxtLayout>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })

const route = useRoute()
const config = useRuntimeConfig()

const token = computed(() => route.query.token as string | undefined)

const password = ref('')
const confirm = ref('')
const loading = ref(false)
const error = ref<string | null>(null)
const success = ref(false)

async function handleSubmit() {
  error.value = null

  if (password.value !== confirm.value) {
    error.value = 'Les mots de passe ne correspondent pas'
    return
  }

  if (password.value.length < 8) {
    error.value = 'Le mot de passe doit contenir au moins 8 caractères'
    return
  }

  loading.value = true

  try {
    await $fetch(`${config.public.apiBase}/api/v1/auth/reset-password`, {
      method: 'POST',
      body: { token: token.value, new_password: password.value },
      headers: { 'Content-Type': 'application/json' },
    })
    success.value = true
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Une erreur est survenue'
  } finally {
    loading.value = false
  }
}
</script>
