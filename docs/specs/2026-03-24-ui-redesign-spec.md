# Smiled.IO UI Redesign Spec

**Date**: 2026-03-24
**Status**: Approved
**Decisions**: Clinical Premium + warm touches, Sidebar classique amelioree, Teal Medical, tous les 10 patterns UX

---

## Design Decisions

| Decision | Choice |
|----------|--------|
| Scope | B — Restructuration UX + reskin |
| Ambiance | Clinical Premium + warm touches |
| Navigation | Sidebar classique amelioree (shadcn-vue Sidebar, collapsible, grouped) |
| Colors | Teal Medical (primary #0D9488, sidebar dark #0F172A) |
| Typography | Inter 14px base, tabular numbers |
| UX Patterns | All 10: Cmd+K, skeletons, sheet/drawer, sonner, breadcrumbs, patient header, data tables, combobox, micro-animations, empty states |

## Color Tokens

### Primary (Teal)
| Token | Hex |
|-------|-----|
| --primary | #0D9488 |
| --primary-light | #14B8A6 |
| --primary-lighter | #99F6E4 |
| --primary-lightest | #F0FDFA |
| --primary-dark | #0F766E |
| --primary-foreground | #FFFFFF |

### Neutrals (Warm Slate)
| Token | Hex |
|-------|-----|
| --background | #FAFAFA |
| --card | #FFFFFF |
| --foreground | #0F172A |
| --muted | #F1F5F9 |
| --muted-foreground | #64748B |
| --border | #E2E8F0 |
| --sidebar-bg | #0F172A |
| --sidebar-fg | #F1F5F9 |

### Semantic
| Token | Hex |
|-------|-----|
| --success | #059669 |
| --warning | #D97706 |
| --destructive | #DC2626 |
| --info | #2563EB |

## Type Scale
| Level | Size | Weight |
|-------|------|--------|
| Display | 30px | 700 |
| H1 | 24px | 600 |
| H2 | 20px | 600 |
| H3 | 16px | 600 |
| Body | 14px | 400 |
| Caption | 12px | 500 |
| Overline | 11px | 600 |

## Spacing & Radius
- Spacing: 4px base (2, 4, 8, 12, 16, 20, 24, 32, 40)
- Border radius: sm=4px, default=8px, lg=12px, xl=16px, full=9999px
- Shadows: xs, sm, md, lg (defined in research report)

## shadcn-vue Components to Install
Sidebar, Command, Table+TanStack, Sheet, Breadcrumb, Skeleton, Sonner/Toast, Combobox, Badge, Tabs, Popover, DropdownMenu, AlertDialog, Avatar, Separator, ScrollArea

## Pages to Redesign
1. Default layout (sidebar + header + breadcrumbs)
2. Login / forgot-password / reset-password
3. Dashboard (index)
4. Patient list
5. Patient detail layout + all sub-pages
6. Actes, Materiaux, Teintes (reference tables)
7. Cabinet settings + users

## New Global Components
- CommandPalette.vue (Cmd+K)
- AppSidebar.vue (shadcn Sidebar)
- PatientHeader.vue (persistent patient context)
- DataTable.vue (reusable TanStack wrapper)
- AppBreadcrumb.vue (dynamic breadcrumbs)
- EmptyState.vue (icon + text + CTA)
- PageSkeleton.vue (loading states)
