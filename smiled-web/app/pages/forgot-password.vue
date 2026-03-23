<template>
  <NuxtLayout name="auth">
    <!-- Branding -->
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold text-primary">Smiled.IO</h1>
      <p class="text-muted-foreground mt-1">Réinitialisation du mot de passe</p>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Mot de passe oublié</CardTitle>
        <CardDescription>
          Entrez votre adresse email pour recevoir un lien de réinitialisation
        </CardDescription>
      </CardHeader>

      <CardContent>
        <div v-if="sent" class="space-y-4">
          <Alert>
            <AlertDescription>
              Si un compte existe pour cette adresse, un email de réinitialisation vous a été envoyé.
            </AlertDescription>
          </Alert>
          <Button variant="outline" class="w-full" @click="reset">
            Envoyer à nouveau
          </Button>
        </div>

        <form v-else class="space-y-4" @submit.prevent="handleSubmit">
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

          <Alert v-if="error" variant="destructive">
            <AlertDescription>{{ error }}</AlertDescription>
          </Alert>

          <Button type="submit" class="w-full" :disabled="loading">
            <Icon v-if="loading" name="lucide:loader-2" class="w-4 h-4 mr-2 animate-spin" />
            Envoyer le lien
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

const config = useRuntimeConfig()

const email = ref('')
const loading = ref(false)
const error = ref<string | null>(null)
const sent = ref(false)

async function handleSubmit() {
  error.value = null
  loading.value = true

  try {
    await $fetch(`${config.public.apiBase}/api/v1/auth/forgot-password`, {
      method: 'POST',
      body: { email: email.value },
      headers: { 'Content-Type': 'application/json' },
    })
    sent.value = true
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Une erreur est survenue'
  } finally {
    loading.value = false
  }
}

function reset() {
  sent.value = false
  email.value = ''
}
</script>
