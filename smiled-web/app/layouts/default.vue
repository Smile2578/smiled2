<template>
  <SidebarProvider>
    <AppSidebar />
    <SidebarInset>
      <header class="flex h-14 shrink-0 items-center gap-2 border-b px-4">
        <SidebarTrigger class="-ml-1" />
        <Separator orientation="vertical" class="mr-2 h-4" />
        <AppBreadcrumb />
        <div class="ml-auto flex items-center gap-2">
          <Button
            variant="outline"
            size="sm"
            class="h-8 gap-2 text-muted-foreground"
            @click="openCommandPalette"
          >
            <Search class="size-4" />
            <span class="hidden md:inline text-xs">Rechercher...</span>
            <kbd class="pointer-events-none hidden h-5 select-none items-center gap-1 rounded border bg-muted px-1.5 font-mono text-[10px] font-medium opacity-100 md:inline-flex">
              <span class="text-xs">{{ metaKey }}</span>K
            </kbd>
          </Button>
        </div>
      </header>
      <main class="flex-1 overflow-auto p-6">
        <slot />
      </main>
    </SidebarInset>
    <CommandPalette ref="commandPaletteRef" />
  </SidebarProvider>
</template>

<script setup lang="ts">
import { Search } from 'lucide-vue-next'

const commandPaletteRef = ref<{ open: boolean } | null>(null)

const metaKey = computed(() => {
  if (typeof navigator === 'undefined') return '⌘'
  return navigator.platform?.includes('Mac') ? '⌘' : 'Ctrl+'
})

function openCommandPalette() {
  if (commandPaletteRef.value) {
    commandPaletteRef.value.open = true
  }
}
</script>
