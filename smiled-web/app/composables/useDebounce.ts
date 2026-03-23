export function useDebounce<T>(value: Ref<T>, delay: number): Ref<T> {
  const debouncedValue = ref<T>(value.value) as Ref<T>

  let timer: ReturnType<typeof setTimeout> | null = null

  watch(value, (newVal) => {
    if (timer) clearTimeout(timer)
    timer = setTimeout(() => {
      debouncedValue.value = newVal
    }, delay)
  })

  return debouncedValue
}
