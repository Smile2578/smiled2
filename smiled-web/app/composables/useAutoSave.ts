import type { Ref } from 'vue'

export function useAutoSave<T>(
  data: Ref<T>,
  saveFn: (d: T) => Promise<void>,
  delay = 1500,
) {
  const saving = ref(false)
  const lastSavedAt = ref<Date | null>(null)
  const error = ref<string | null>(null)
  const debouncedData = useDebounce(data, delay)

  watch(debouncedData, async (newData) => {
    if (!newData) return
    saving.value = true
    error.value = null
    try {
      await saveFn(toRaw(newData) as T)
      lastSavedAt.value = new Date()
    } catch (err) {
      error.value = err instanceof Error ? err.message : 'Erreur de sauvegarde'
    } finally {
      saving.value = false
    }
  }, { deep: true })

  return { saving: readonly(saving), lastSavedAt: readonly(lastSavedAt), error: readonly(error) }
}
