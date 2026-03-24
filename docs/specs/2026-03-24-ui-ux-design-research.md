# Smiled.IO -- UI/UX Design Research Report

**Date**: 2026-03-24
**Author**: Claude (Opus 4.6) + Web Research
**Status**: v1
**Objective**: Define the visual direction and design system for Smiled.IO based on competitive analysis and modern design trends

---

## 1. Competitive UI Analysis

### 1.1 French Competitors

#### Veasy (Cegedim) -- Market Leader Cloud
- **UI Character**: Clean cloud interface, color-coded appointment types, drag-and-drop scheduling
- **Strengths**: Functional dashboard with analytics, treatment templates, inventory tracking
- **Weaknesses**: Reviews cite "dated" feel despite being cloud-native, generic SaaS look without dental identity
- **Design Language**: Standard blue enterprise SaaS, tabs-based navigation, dense forms
- **Rating**: 4.0/5 on Capterra (functional but not inspiring)

#### Julie / JULiA -- Incumbent Champion
- **UI Character**: JULiA (2025 rewrite) claims "clean and fluid interface" with centralized dashboard
- **Strengths**: AI natively integrated (image processing, dictation, analytics), regulatory compliance
- **Weaknesses**: 42-year legacy brand -- even the rewrite carries old-school perception
- **Design Language**: Traditional desktop-app-in-browser aesthetic, functional over beautiful
- **Takeaway**: JULiA proves the market is ready for modern UI -- but execution still feels like a clinical tool, not a premium product

#### LOGOSw -- Desktop Holdout
- **UI Character**: Classic Windows desktop application (client lourd)
- **Strengths**: Deep clinical features (ODF, paro, DICOM), fast support (30min resolution)
- **Weaknesses**: Visually stuck in the 2000s, no cloud option, no responsive design
- **Design Language**: Traditional toolbar/menu bar paradigm, small fonts, crowded screens
- **Takeaway**: Represents exactly what Smiled.IO should NOT look like

#### Matisse -- Modern Newcomer
- **UI Character**: Claims "intuitive and modern interface", responsive on all devices including tablets
- **Strengths**: 50+ integrations, AI agenda, hybrid cloud with offline mode
- **Weaknesses**: New (2024), limited track record, unclear if UI delivers on promises
- **Design Language**: Modern SaaS aesthetic, likely React/Vue-based
- **Takeaway**: Closest French competitor in terms of design ambition -- but not open source

#### Giantix -- Pre-Launch AI-First
- **UI Character**: Marketing emphasizes AI dashboards, analytics, panoramic analysis
- **Strengths**: Free during beta, AI-first approach, modern marketing
- **Weaknesses**: Pre-launch (2025), unproven, limited feature set
- **Takeaway**: Watch for their UI direction but they are behind on features

### 1.2 International Competitors

#### Curve Dental -- Design Gold Standard
- **4.7/5 on Capterra (440+ reviews)** -- highest-rated for UI
- **UI Character**: Clean browser-based interface, consistently praised for ease of use
- **Key Pattern**: Odontogram always displayed alongside treatment plan -- complete visual reference
- **Strengths**: Color-coded charting, intuitive onboarding ("up and running in days"), 30% increase in case acceptance due to visual patient communication
- **Design Language**: Light, airy, generous whitespace, modern sans-serif, subtle color coding
- **Takeaway**: THE benchmark for "dental software that feels premium". Study their charting UX.

#### Open Dental -- Open Source Reference
- **UI Character**: Functional Windows desktop app with customizable layouts
- **Strengths**: Extensive customization (Chart Module Layout via Sheets), module-based architecture, integrated imaging
- **Weaknesses**: Dated visual design, Windows-only look, steep learning curve
- **Design Language**: Traditional toolbar/grid paradigm, high information density
- **Takeaway**: Feature inspiration (customizable charting, procedure buttons) but NOT design inspiration

#### Dentally (Henry Schein One) -- Cloud Pioneer
- **UI Character**: Modern cloud-first, "intuitive and built for modern practices"
- **Strengths**: Daily micro-updates, cross-platform (Windows, Mac, iPad), cloud-saved x-rays + AI
- **Design Language**: Clean, minimalist, designed in collaboration with dentists
- **Takeaway**: Good example of "less is more" -- proves dental software can be simple

#### Pearl -- AI-First Premium
- **UI Character**: "Modern and futuristic" interface with color-coded diagnostic overlays
- **Key Pattern**: ToothParts visualization -- color-coded tooth structures for patient education
- **Strengths**: Real-time performance dashboards by provider/location, overlay-based diagnosis
- **Design Language**: Dark-mode-friendly, high-contrast medical imaging aesthetic, data viz focus
- **Takeaway**: Best example of "dental software as a premium AI product". Visual patient education is a key differentiator.

#### Tab32 -- Enterprise Cloud
- **UI Character**: "Clean and neatly organized" with minimal navigation layers
- **Strengths**: Single integrated dashboard, KPI analytics, enterprise-grade on Google Cloud
- **Design Language**: Enterprise SaaS, dashboard-centric, analytics-heavy
- **Takeaway**: Good reference for multi-praticien analytics dashboard

---

## 2. Key UI Patterns Across Top Dental Software

### 2.1 Universal Patterns (present in ALL top products)

| Pattern | Implementation | Priority for Smiled.IO |
|---------|---------------|----------------------|
| **Persistent dental chart** | Always visible alongside treatment plan (Curve, Open Dental) | CRITICAL |
| **Color-coded status** | Distinct colors for existing/planned/completed/referred (all products) | CRITICAL |
| **Patient summary header** | Name, age, allergies, last visit -- always visible on patient pages | HIGH |
| **Quick action buttons** | One-click status changes, procedure shortcuts (Curve, Open Dental) | HIGH |
| **Tabbed patient record** | Chart / History / Treatment Plan / Documents / Billing tabs | HIGH |
| **Search-driven navigation** | Command palette or global search bar (modern products) | HIGH |
| **Contextual popovers** | Click tooth -> popover with quick actions (vs. full page navigation) | MEDIUM |
| **Drag-and-drop scheduling** | Appointment management with visual time blocks | Phase 2+ |

### 2.2 Differentiating Patterns (present in best-in-class only)

| Pattern | Who Does It | Impact |
|---------|------------|--------|
| **Odontogram + PDT side-by-side** | Curve Dental | 30% case acceptance increase |
| **Color-coded tooth structure visualization** | Pearl (ToothParts) | Patient education & trust |
| **AI diagnostic overlays on X-rays** | Pearl, Allisone, Giantix | Clinical confidence |
| **Real-time KPI dashboard** | Tab32, Pearl, Matisse | Practice management |
| **Role-specific interface** | Enterprise products | Reduces cognitive load |
| **Medical alert persistent banner** | Best EHR systems | Patient safety |

### 2.3 Anti-Patterns to Avoid

| Anti-Pattern | Who Suffers From It | Why It Fails |
|-------------|-------------------|-------------|
| Toolbar overload (20+ icons) | LOGOSw, Open Dental | Cognitive overload, steep learning curve |
| Deep menu nesting (3+ levels) | Julie legacy, Dentrix | Slow workflows, lost context |
| Modal-heavy workflows | Older desktop apps | Disrupts flow, loses context |
| Tiny fixed-size windows | Desktop apps | Not responsive, wastes screen estate |
| Monochrome dense tables | Enterprise EHRs | Clinical data becomes unreadable |
| Login-page-quality design everywhere | Generic SaaS | No sense of "built for dentists" |

---

## 3. Healthcare SaaS Design Trends 2025-2026

### 3.1 Macro Trends

1. **Command Palettes as Standard** (2026): No longer a power-user feature. Every SaaS with 10+ features needs one. Type what you want to do -- zero navigation friction. (Source: SaaSUI 2026 trends)

2. **Role-Based Experience Design**: Beyond permissions into fully different interfaces per role. A secretaire sees scheduling; a praticien sees clinical charts. Not just hiding buttons -- genuinely different layouts.

3. **Progressive Disclosure**: The best 2026 products "carefully sequence when users encounter features". Surface the 80% case immediately; tuck the 20% behind expandable sections.

4. **Data-Dense but Organized**: Healthcare cannot afford minimalism. Clinicians need high information density. The solution: strong visual hierarchy, consistent spacing, typographic scale, and strategic color coding. (Source: AHRQ research on dense clinical displays)

5. **Predictive Dashboards**: Dashboards evolve from "here is your data" to "here is what you should do next". Alerts, risk scores, suggested next steps.

6. **Warm + Human, Not Cold + Clinical**: 2026 trend away from sterile medical blue toward "warm paper + slate UI + one calming accent". Less clinical, more human. (Source: Elegant Themes 2026)

7. **Micro-Interactions & Subtle Animation**: Smooth transitions, hover states, loading skeletons, toast notifications. These small details make a product feel "alive" and premium vs. static and dated.

### 3.2 Component Trends

| Component | 2026 Best Practice |
|-----------|-------------------|
| **Sidebar** | Collapsible, icon-only mode, grouped by domain (Clinical / Admin / Settings) |
| **Command Palette** | Cmd+K, searches patients + actions + pages, fuzzy matching |
| **Data Tables** | Sortable, filterable, column toggles, row actions, pagination + infinite scroll |
| **Cards** | Soft shadows, generous padding, subtle borders, rounded corners (8-12px) |
| **Forms** | Inline validation, auto-save indicator, field-level help text |
| **Modals** | Reserved for confirmations only; complex workflows use sheets/drawers |
| **Sheets/Drawers** | Slide-in panels for detail views (patient detail, tooth detail) |
| **Toasts** | Non-blocking feedback for all actions (save, delete, error) |
| **Skeleton Loaders** | Animated placeholders during data fetch (vs. spinner) |
| **Breadcrumbs** | Patient > Fiche > Schema dentaire -- always show context |

---

## 4. Recommended Design Direction for Smiled.IO

### 4.1 Design Philosophy: "Clinical Precision, Startup Soul"

Smiled.IO should feel like **Notion meets a dental clinic** -- clean, modern, data-rich but never overwhelming. The goal is a product that:

1. A 30-year-old dentist looks at and thinks "finally, a dental software from this century"
2. A 55-year-old practitioner can learn in under an hour
3. A patient sees on the screen and feels confident in their care

**Positioning**: Between Curve Dental's simplicity and Tab32's data richness. NOT the clinical sterility of old EHRs. NOT the playful consumer aesthetic of patient apps.

### 4.2 Design Principles

| Principle | Meaning | Example |
|-----------|---------|---------|
| **Density with Clarity** | Show lots of data without feeling cluttered | Dental chart + PDT + alerts on one screen, organized with clear hierarchy |
| **One-Click Workflows** | Reduce clicks to minimum for common actions | Click tooth -> popover -> one click to change status |
| **Always-Visible Context** | Never lose "where am I" and "who is this patient" | Persistent patient header + breadcrumbs + sidebar state |
| **Color as Information** | Color is functional, not decorative | Tooth status colors, alert severity, treatment phase |
| **Progressive Complexity** | Simple by default, powerful when needed | Basic view first; "Advanced" expandable sections for power users |

### 4.3 Layout Architecture

```
+--------------------------------------------------+
| [Logo] [Global Search Cmd+K] [Notif] [User Menu] | <- Top bar (56px)
+------+-------------------------------------------+
|      |  [Breadcrumbs: Cabinet > Patient > Dossier]|
| Side |                                            |
| bar  |  +---- Main Content Area --------+         |
|      |  |                               |         |
| [Nav |  |  Context-dependent content    |         |
|  ico |  |  (dental chart, forms, tables)|         |
|  ns] |  |                               |  [Opt.  |
|      |  +-------------------------------+  Detail |
|      |                                    Panel]  |
|      |  [Status Bar: Auto-save indicator, alerts] |
+------+-------------------------------------------+
```

**Key decisions**:
- **Collapsible sidebar** (icons-only mode for maximum chart space)
- **Top bar** with global search (Cmd+K command palette)
- **Optional detail panel** (right-side sheet) for tooth detail, PDT line detail
- **Persistent patient header** when inside a patient context
- **Status bar** at bottom for auto-save feedback and medical alerts

---

## 5. Color Palette

### 5.1 Primary Palette: "Teal Trust"

The research is clear: **teal/blue dominates healthcare** (62% of healthcare sites use blue or teal above the fold). But pure blue feels generic (every bank, every enterprise SaaS). Teal distinguishes Smiled.IO as healthcare-specific while feeling modern.

| Token | Hex | HSL | Usage |
|-------|-----|-----|-------|
| `--primary` | `#0D9488` | 175 89% 32% | Primary actions, links, active states |
| `--primary-light` | `#14B8A6` | 173 80% 40% | Hover states, highlights |
| `--primary-lighter` | `#99F6E4` | 167 85% 78% | Badges, subtle backgrounds |
| `--primary-lightest` | `#F0FDFA` | 166 76% 97% | Card backgrounds, tinted areas |
| `--primary-dark` | `#0F766E` | 175 84% 26% | Pressed states, dark accents |
| `--primary-foreground` | `#FFFFFF` | 0 0% 100% | Text on primary color |

### 5.2 Neutral Palette: "Warm Slate"

Avoid pure gray (too cold/clinical). Use slate with a hint of warmth.

| Token | Hex | HSL | Usage |
|-------|-----|-----|-------|
| `--background` | `#FAFAFA` | 0 0% 98% | Page background |
| `--card` | `#FFFFFF` | 0 0% 100% | Card surfaces |
| `--foreground` | `#0F172A` | 222 47% 11% | Primary text |
| `--muted` | `#F1F5F9` | 210 40% 96% | Muted backgrounds |
| `--muted-foreground` | `#64748B` | 215 16% 47% | Secondary text |
| `--border` | `#E2E8F0` | 214 32% 91% | Borders, dividers |
| `--sidebar-bg` | `#0F172A` | 222 47% 11% | Dark sidebar background |
| `--sidebar-fg` | `#F1F5F9` | 210 40% 96% | Sidebar text |

### 5.3 Semantic Colors

| Token | Hex | Usage |
|-------|-----|-------|
| `--success` | `#059669` (Emerald 600) | Completed treatments, positive feedback |
| `--warning` | `#D97706` (Amber 600) | Medical alerts, pending items |
| `--destructive` | `#DC2626` (Red 600) | Errors, deletions, critical alerts |
| `--info` | `#2563EB` (Blue 600) | Informational badges, links |

### 5.4 Clinical Color Coding (Dental Chart)

Based on the universal standard across Curve, Open Dental, and other dental software:

| Status | Color | Hex | Reasoning |
|--------|-------|-----|-----------|
| Healthy / Present | Light gray | `#E2E8F0` | Neutral baseline |
| Existing restoration | Blue | `#3B82F6` | Already done (cool, stable) |
| Treatment planned | Orange/amber | `#F59E0B` | Needs attention (warm, active) |
| Completed today | Green | `#10B981` | Just done (positive, fresh) |
| Extracted / Missing | Dark gray | `#6B7280` | Absence (muted) |
| Carious / Pathology | Red | `#EF4444` | Problem (urgent, attention) |
| Endodontic | Purple | `#8B5CF6` | Specialty treatment |
| Prosthetic | Teal | `#14B8A6` | Prosthetic work (brand alignment) |

### 5.5 Why NOT Pure Blue

Your current palette (`--primary: 221.2 83.2% 53.3%` = approximately `#3B82F6`) is standard Tailwind blue. This is the default shadcn color. Problems:
1. Indistinguishable from Veasy, Dentrix, every enterprise SaaS
2. No healthcare-specific identity
3. Conflicts with "existing restoration" blue in the dental chart
4. Blue fatigue -- users have seen this exact blue a thousand times

**Recommendation**: Shift primary from blue to teal. Keep blue as a semantic/info color only.

---

## 6. Typography

### 6.1 Font Selection: Inter

Your current choice of **Inter** is excellent and should stay. Reasons:

- **Tall x-height**: More readable at small sizes without increasing font size -- critical for data-dense clinical screens
- **Variable font**: Performance optimized, fewer HTTP requests
- **Designed for screens**: Purpose-built for UI, unlike fonts adapted from print
- **Used by NASA and medical equipment**: Proven in high-stakes, data-dense contexts
- **Universal support**: Google Fonts, self-hosted, system font fallback

Alternative consideration: **Geist** (by Vercel) is excellent for developer tools but slightly too "tech startup" for a medical product. Inter is the safer choice for dental professionals.

### 6.2 Type Scale

| Level | Size | Weight | Line Height | Usage |
|-------|------|--------|-------------|-------|
| Display | 30px / 1.875rem | 700 (Bold) | 1.2 | Page titles (rare) |
| H1 | 24px / 1.5rem | 600 (Semibold) | 1.3 | Section headers |
| H2 | 20px / 1.25rem | 600 (Semibold) | 1.4 | Card titles |
| H3 | 16px / 1rem | 600 (Semibold) | 1.5 | Sub-section titles |
| Body | 14px / 0.875rem | 400 (Regular) | 1.5 | Default text, table cells |
| Body Small | 13px / 0.8125rem | 400 (Regular) | 1.5 | Dense tables, metadata |
| Caption | 12px / 0.75rem | 500 (Medium) | 1.4 | Labels, badges, timestamps |
| Overline | 11px / 0.6875rem | 600 (Semibold) | 1.4 | Uppercase labels, section dividers |

### 6.3 Key Rules

- **14px base** (not 16px) -- dental software is data-dense, 16px wastes space
- **Tabular numbers** (`font-feature-settings: "tnum"`) for prices, dates, FDI numbers
- **Letter-spacing +0.02em on overlines** for readability at small sizes
- **Never below 11px** -- accessibility minimum for clinical data

---

## 7. Spacing & Sizing System

### 7.1 Spacing Scale (4px base)

| Token | Value | Usage |
|-------|-------|-------|
| `space-0.5` | 2px | Icon gaps, tight inline spacing |
| `space-1` | 4px | Minimum gap between related elements |
| `space-2` | 8px | Default gap inside compact components |
| `space-3` | 12px | Card internal padding (compact) |
| `space-4` | 16px | Standard card padding, section gaps |
| `space-5` | 20px | Generous padding |
| `space-6` | 24px | Section separators, major gaps |
| `space-8` | 32px | Page-level spacing |
| `space-10` | 40px | Top-level section gaps |

### 7.2 Border Radius

| Token | Value | Usage |
|-------|-------|-------|
| `--radius-sm` | 4px | Small elements (badges, chips) |
| `--radius` | 8px | Default (buttons, inputs, cards) |
| `--radius-lg` | 12px | Large cards, modals |
| `--radius-xl` | 16px | Feature cards, hero sections |
| `--radius-full` | 9999px | Avatars, circular buttons |

### 7.3 Shadows

| Token | Value | Usage |
|-------|-------|-------|
| `--shadow-xs` | `0 1px 2px rgba(0,0,0,0.05)` | Subtle elevation (dropdowns) |
| `--shadow-sm` | `0 1px 3px rgba(0,0,0,0.1), 0 1px 2px rgba(0,0,0,0.06)` | Cards, inputs |
| `--shadow-md` | `0 4px 6px rgba(0,0,0,0.1), 0 2px 4px rgba(0,0,0,0.06)` | Popovers, floating elements |
| `--shadow-lg` | `0 10px 15px rgba(0,0,0,0.1), 0 4px 6px rgba(0,0,0,0.05)` | Modals, sheets |

---

## 8. Component Patterns for Smiled.IO

### 8.1 Must-Have Components (Phase 1-2)

| Component | Implementation | Priority |
|-----------|---------------|----------|
| **Command Palette** (Cmd+K) | shadcn-vue `Command` component. Search patients by name/INS, navigate pages, trigger actions ("Nouveau patient", "Schema dentaire") | CRITICAL |
| **Collapsible Sidebar** | shadcn-vue `Sidebar` with icon-only mode. Groups: Clinique, Administration, Parametres | CRITICAL |
| **Patient Header** | Sticky header on all patient sub-pages: nom, prenom, age, photo, medical alerts badge, last visit | CRITICAL |
| **Data Table** | shadcn-vue `Table` + TanStack Table. Sortable, filterable, paginated. For patients, actes, PDT lines | CRITICAL |
| **Breadcrumbs** | Always visible below top bar. Shows navigation context | HIGH |
| **Toast Notifications** | shadcn-vue `Toast` (Sonner). All user feedback via non-blocking toasts | HIGH |
| **Sheet / Drawer** | Right-side slide-in panel for tooth detail, PDT line edit. Keeps chart visible | HIGH |
| **Auto-Save Indicator** | Subtle status in bottom bar: "Saving..." -> checkmark + "Saved at 14:32" | HIGH |
| **Medical Alert Banner** | Persistent amber/red bar on patient pages: AVK, allergies, endocardite, bisphosphonates | HIGH |
| **Skeleton Loaders** | Animated placeholder shapes during data loading (not spinners) | MEDIUM |
| **Badge** | Color-coded status badges for treatment phases, roles, alerts | MEDIUM |
| **Tabs** | For patient sub-pages: Fiche / Schema / PDT / Documents / Historique | MEDIUM |

### 8.2 Dental-Specific Components

| Component | Design Notes |
|-----------|-------------|
| **Dental Chart (Canvas 2D)** | Full-width by default, responsive. Each tooth clickable. Color-coded by status. FDI numbering visible. Upper arch on top, lower on bottom. |
| **Tooth Popover** | On tooth click: popover with quick status buttons, face selector, notes field, "Details" link to full sheet |
| **PDT Table** | Inline-editable table with autocomplete for actes (search by code CCAM or libelle), prix, materiaux, teinte |
| **Paro Chart** | 6-point probing depth visualization, color-coded by severity (green < 3mm, yellow 4-5mm, red > 5mm) |
| **Questionnaire Form** | Section-based accordion: Antecedents generaux / Allergies / Traitements / Habitudes. Toggle + detail pattern |

### 8.3 Component Interaction Patterns

**Tooth Selection Flow** (optimized from Curve Dental research):
```
1. Click tooth on chart
   -> Popover appears at tooth position
   -> Quick status buttons (Sain, Carie, Absent, Restaure...)
   -> Click status = instant save + color update on chart
   -> "Details" link opens right-side Sheet with full tooth info

2. Keyboard alternative:
   -> Arrow keys navigate between teeth
   -> Enter/Space opens popover
   -> Number keys for quick status (1=Sain, 2=Carie, etc.)
   -> Escape closes, moves to next tooth
```

**Patient Search Flow** (command palette pattern):
```
1. Cmd+K opens command palette
2. Type patient name (fuzzy search via pg_trgm)
3. Results show: name, DOB, last visit, photo
4. Enter = navigate to patient dossier
5. Alt+Enter = open in new tab
```

---

## 9. What Makes Dental Software Feel "Premium" vs "Generic"

### 9.1 Premium Signals

| Signal | Implementation | Impact |
|--------|---------------|--------|
| **Custom dental chart visualization** | Canvas 2D with smooth rendering, tooth hover effects, face-level detail | "This was built FOR dentists" |
| **Instant feedback** | Optimistic updates, skeleton loaders, smooth transitions | "This is fast and responsive" |
| **Contextual help** | Tooltips on CCAM codes, inline nomenclature references | "This understands my workflow" |
| **Data density without clutter** | Well-structured tables, strategic whitespace, clear hierarchy | "I can see everything I need" |
| **Consistent color language** | Same status colors everywhere (chart, PDT, badges, tables) | "I never have to think about what colors mean" |
| **Micro-interactions** | Button press feedback, hover states on teeth, toast slide-in | "This feels polished and alive" |
| **Empty states** | Illustrated empty states with clear CTAs ("No treatment plan yet. Create one?") | "This was designed with care" |
| **Error recovery** | Undo toast on delete, auto-save with conflict resolution | "I trust this with my data" |

### 9.2 Generic Red Flags

| Red Flag | What It Signals |
|----------|----------------|
| Default browser form styling | "We did not invest in design" |
| Spinner on every action | "Our architecture is slow" |
| Modals for everything | "We did not think about workflow" |
| All-blue everything | "We used the default template" |
| Lorem ipsum anywhere | "This is not production quality" |
| No loading states | "We did not consider UX" |
| Identical view for all roles | "We did not understand our users" |
| PDF exports that look different from the app | "Design was an afterthought" |

---

## 10. Specific Recommendations for Smiled.IO

### 10.1 Immediate Changes (Current Phase)

1. **Migrate primary color from blue (#3B82F6) to teal (#0D9488)**
   - Update `--primary` CSS variable
   - Update `brand` palette in tailwind.config.ts
   - Sidebar can stay dark (slate `#0F172A`) -- teal accents on active items

2. **Add clinical status colors to design system**
   - Define the 8 dental chart status colors as CSS variables
   - Use consistently across chart, PDT table, badges

3. **Implement Command Palette** (Cmd+K)
   - shadcn-vue `Command` component
   - Search: patients, pages, actions
   - This single feature signals "modern SaaS" immediately

4. **Add Breadcrumbs** below top bar
   - Simple but critical for "where am I" context in deep patient records

5. **Switch base font size to 14px**
   - Enable `font-feature-settings: "tnum"` for tabular numbers
   - Increase information density without sacrificing readability

### 10.2 Phase 2 Design Polish

1. **Skeleton loaders** for all data fetches (replace spinners)
2. **Empty states** with illustrations for all list views
3. **Toast system** (Sonner) for all user feedback
4. **Auto-save indicator** in status bar
5. **Patient header** with medical alerts banner
6. **Sheet/Drawer** for tooth detail (replace full-page navigation)

### 10.3 Phase 3 Design Ambitions

1. **Dark mode** (already have CSS variables structure for it)
2. **Keyboard-driven workflow** (Vim-like shortcuts for power users)
3. **Patient-facing view** (simplified, large font, visual treatment explanations)
4. **Printable treatment plan** that matches the app's design language
5. **Mobile-optimized views** for tablet use in treatment rooms

---

## 11. shadcn-vue Templates & Resources

### 11.1 Recommended Starting Points

| Template | URL | Relevance |
|----------|-----|-----------|
| **Nuxt Shadcn-Vue Dashboard** | nuxt-shadcn-dashboard.vercel.app | Best Nuxt 4 + shadcn-vue dashboard base. Sidebar, command palette, dark mode. |
| **shadcn-vue Official Dashboard** | shadcn-vue.com/examples/dashboard | Official example with analytics dashboard layout |
| **Shadcn Admin (satnaing)** | github.com/satnaing/shadcn-admin | 10+ pages, collapsible sidebar, command search, RTL support |
| **Shadcn UI Kit Hospital** | shadcnuikit.com/dashboard/hospital-management | Purpose-built hospital management dashboard |

### 11.2 Specific Components to Adopt from shadcn-vue

- `Sidebar` (collapsible, with groups)
- `Command` (command palette)
- `Table` (with TanStack Table integration)
- `Sheet` (slide-in panels)
- `Toast` / Sonner (notifications)
- `Tabs` (patient sub-navigation)
- `Badge` (status indicators)
- `Breadcrumb` (navigation context)
- `Skeleton` (loading states)
- `Popover` (tooth quick actions)
- `DropdownMenu` (row actions in tables)
- `AlertDialog` (destructive confirmations only)

### 11.3 Additional UI Libraries to Consider

| Library | Purpose | URL |
|---------|---------|-----|
| **inspira-ui** | Extra shadcn-compatible animated components | inspira-ui.com |
| **Recharts / vue-chartjs** | KPI dashboards, practice analytics | -- |
| **lucide-vue-next** | Icon set (already likely in use via shadcn-vue) | -- |

---

## 12. Summary: The Smiled.IO Design Formula

```
Smiled.IO = Curve Dental's simplicity
          + Tab32's data richness
          + Pearl's visual innovation
          + shadcn-vue's component quality
          + Teal trust palette
          + Inter typography
          + Command palette navigation
          + Progressive disclosure
          + Dental-first color coding
```

**In one sentence**: A data-dense clinical tool that feels as polished as a modern startup product, instantly recognizable as "built for dentists" through its teal identity and dental chart-centric workflow.

---

## Sources

### Competitor Reviews & Comparisons
- [Best Dental Practice Management Software 2026 -- Curve Dental](https://www.curvedental.com/dental-blog/best-dental-practice-management-software-2026)
- [Best Dental Software 2026 -- Cherry](https://withcherry.com/blog/dental-practice-management-software)
- [Dentrix vs Eaglesoft -- Software Advice](https://www.softwareadvice.com/dental/dentrix-profile/vs/patterson-dental-eaglesoft/)
- [Top 19 Dental Software 2026 -- mConsent](https://mconsent.net/blog/top-dental-software/)
- [Best Dental PMS US 2026 -- Pabau](https://pabau.com/blog/top-7-dental-practice-software-in-the-us-2026-guide)
- [Top 10 Dental Software 2026 -- Adit](https://adit.com/top-10-dental-software-in-2026)
- [Veasy -- Capterra Reviews](https://www.capterra.com/p/207643/Veasy/)
- [Pearl AI Dental](https://hellopearl.com/)
- [Pearl Practice Intelligence](https://hellopearl.com/products/practice-intelligence)
- [Tab32 Cloud Dental](https://tab32.com/)
- [Dentally Cloud Software](https://www.dentally.com/en-gb/)
- [Curve Dental Charting](https://www.curvedental.com/dental-charting-software)
- [Open Dental Manual](https://www.opendental.com/manual/mainmenu.html)
- [Open Dental Chart Colors](https://opendental.com/manual/definitionschartgraphiccolors.html)
- [JULiA -- Julie Solutions](https://www.julie.fr/article-julia-reinvente-le-pilotage-des-activites-de-chirurgie-dentaire-avec-une-approche-nouvelle-generation)
- [Julie -- Capterra](https://www.capterra.com/p/207109/Julie/)
- [Matisse Dentaire](https://www.matisse-dentaire.com/)
- [Matisse Features](https://www.matisse-dentaire.com/logiciel-dentaire/fonctionnalites/)

### Design & Dashboard Inspiration
- [Dental Dashboard -- Multipurpose Themes](https://multipurposethemes.com/blog/dental-dashboard-the-digital-brain-of-your-clinic/)
- [Dental Dashboard -- Dribbble](https://dribbble.com/tags/dental-dashboard)
- [Dental UI -- Dribbble](https://dribbble.com/tags/dental-ui)
- [22 Outstanding Dentist App UI Designs](https://ictsolved.github.io/22-outstanding-dentist-app-ui-designs/)
- [Free UI Dental App Kit -- Figma](https://www.figma.com/community/file/1052564559032616008/free-ui-dental-app-kit)
- [React Odontogram](https://peerlist.io/biomathcode/project/react-odontogram)

### Healthcare UX Trends
- [Healthcare UI Design 2026 -- Eleken](https://www.eleken.co/blog-posts/user-interface-design-for-healthcare-applications)
- [Healthcare UX Design 2025 -- Webstacks](https://www.webstacks.com/blog/healthcare-ux-design)
- [Top 7 Healthcare UX/UI Trends 2026 -- Excellent Web World](https://www.excellentwebworld.com/healthcare-ux-ui-design-trends/)
- [50 Healthcare UX/UI Design Trends -- KoruUX](https://www.koruux.com/50-examples-of-healthcare-UI/)
- [Top 10 UX Trends Healthcare 2026 -- UX Studio](https://www.uxstudioteam.com/ux-blog/healthcare-ux)
- [7 SaaS UI Design Trends 2026 -- SaaSUI](https://www.saasui.design/blog/7-saas-ui-design-trends-2026)
- [Data Dense Displays in Healthcare -- AHRQ](https://digital.ahrq.gov/ahrq-funded-projects/use-dense-display-data-and-information-design-principles-primary-care-health)
- [EHR Interface Design 2026 -- Arkenea](https://arkenea.com/blog/ehr-interface/)
- [Dense Clinical Interface -- Medblocks](https://medblocks.com/blog/how-we-turned-a-chaotic-high-information-density-interface-into-a-masterpiece-of-functionality)

### Color & Typography
- [Color Psychology in Dental Design -- Golden Proportions](https://www.goldenproportions.com/blog/web-design/the-psychology-of-color-in-dental-website-design-what-patients-really-see/)
- [Dental Clinic Color Palette -- Master Dent](https://www.masterdentgroup.com/blog/color-guide-for-dental-clinics)
- [Healthcare Color Palette -- Progress](https://www.progress.com/blogs/using-color-psychology-healthcare-web-design)
- [27 Dental Care Color Palettes -- Colorany](https://colorany.com/color-palettes/dental-care-color-palettes/)
- [Medical Color Palette -- Media.io](https://www.media.io/color-palette/medical-color-palette.html)
- [Inter Font](https://rsms.me/inter/)
- [Inter -- Google Fonts](https://fonts.google.com/specimen/Inter)
- [Geist Font -- Vercel](https://vercel.com/font)
- [Best UI Design Fonts 2026 -- Design Monks](https://www.designmonks.co/blog/best-fonts-for-ui-design)
- [Fonts in Healthcare -- Brandcare](https://www.brandcare.net/blog/fonts-in-healthcare/)

### shadcn Templates
- [shadcn-vue Dashboard Examples](https://www.shadcn-vue.com/examples/dashboard)
- [Nuxt Shadcn-Vue Dashboard](https://nuxt-shadcn-dashboard.vercel.app/)
- [25 Best Shadcn Admin Dashboards 2026 -- AdminLTE](https://adminlte.io/blog/shadcn-admin-dashboard-templates/)
- [Shadcn Admin -- GitHub](https://github.com/satnaing/shadcn-admin)
- [Shadcn UI Kit Hospital Management](https://shadcnuikit.com/dashboard/hospital-management)
- [10 Best Healthcare Dashboard Templates 2026 -- AdminLTE](https://adminlte.io/blog/healthcare-dashboard-templates/)

### Design Systems & Tokens
- [Design System SaaS 2026 -- Polara Studio](https://www.polarastudio.fr/blog/comment-construire-un-design-system-saas-en-2026)
- [SaaS Design System Guide -- F1Studioz](https://f1studioz.com/blog/saas-design-system-guide/)
- [5 CSS Style Guides 2026 -- Medium](https://medium.com/@doriansotpyrc/steal-these-5-drop-in-css-style-guides-dominating-2026-web-design-0e5074a66b19)
- [Sidebar Design UX 2026 -- ALF Design](https://www.alfdesigngroup.com/post/improve-your-sidebar-design-for-web-apps)
- [Shadcn Sidebar Components -- WrapPixel](https://www.wrappixel.com/shadcn-sidebar/)

### Digital Dental Trends
- [Top Digital Dental Trends 2026 -- 3Shape](https://www.3shape.com/en/blog/inspiration-on-dentistry/top-digital-dental-trends-to-watch-for-in-2026)
- [Dental Chart Thesis -- Theseus](https://www.theseus.fi/bitstream/handle/10024/795409/Holmnas_Staffan.pdf)
- [Web-Based Dental Rendering -- PMC](https://pmc.ncbi.nlm.nih.gov/articles/PMC8393219/)
