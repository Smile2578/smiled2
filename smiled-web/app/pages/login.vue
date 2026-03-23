<template>
  <NuxtLayout name="auth">
    <!-- Branding -->
    <div class="text-center mb-8">
      <h1 class="text-3xl font-bold text-primary">Smiled.IO</h1>
      <p class="text-muted-foreground mt-1">Connectez-vous à votre espace</p>
    </div>

    <Card>
      <CardHeader>
        <CardTitle>Connexion</CardTitle>
        <CardDescription>
          Entrez vos identifiants pour accéder à votre cabinet
        </CardDescription>
      </CardHeader>

      <CardContent>
        <form class="space-y-4" @submit.prevent="handleLogin">
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

          <div class="space-y-2">
            <Label for="password">Mot de passe</Label>
            <Input
              id="password"
              v-model="password"
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
            Connexion
          </Button>
        </form>
      </CardContent>

      <CardFooter class="flex justify-center">
        <NuxtLink
          to="/forgot-password"
          class="text-sm text-muted-foreground hover:text-primary transition-colors"
        >
          Mot de passe oublié ?
        </NuxtLink>
      </CardFooter>
    </Card>
  </NuxtLayout>
</template>

<script setup lang="ts">
definePageMeta({ layout: false })

const authStore = useAuthStore()
const router = useRouter()

const email = ref('')
const password = ref('')
const loading = ref(false)
const error = ref<string | null>(null)

async function handleLogin() {
  error.value = null
  loading.value = true

  try {
    await authStore.login(email.value, password.value)
    await router.push('/')
  } catch (err) {
    error.value = err instanceof Error ? err.message : 'Identifiants invalides'
  } finally {
    loading.value = false
  }
}
</script>
