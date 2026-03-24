export function usePermissions() {
  const permissions = useState<string[]>('user-permissions', () => [])

  async function loadPermissions() {
    const { apiGet } = useApi()
    try {
      const res = await apiGet<string[]>('/api/v1/me/permissions')
      if (res.success && res.data) permissions.value = res.data
    } catch {
      // permissions stay empty = no access
    }
  }

  function hasPermission(name: string): boolean {
    return permissions.value.includes(name)
  }

  return { permissions, loadPermissions, hasPermission }
}
