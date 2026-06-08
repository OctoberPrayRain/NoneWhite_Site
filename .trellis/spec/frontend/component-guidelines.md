# Component Guidelines

> How components are built in this project.

---

## Overview

Current Vue files use Vue 3 single-file components with `<script setup>` and plain JavaScript. The project does not currently split forms into smaller child components; route pages own their local form state and call shared API/store modules directly.

Prefer simple SFCs over premature abstraction. Extract a child component only when there is real reuse across pages or a route file becomes hard to scan.

---

## Component Structure

Use this structure for current frontend files:

```vue
<script setup>
import { ref } from 'vue'

const status = ref('idle')

function handleSubmit() {
  // validate, call API/store, then set visible status messages
}
</script>

<template>
  <section class="auth-page">
    <h1>登录 NoneWhite</h1>
    <p class="auth-copy">Use project form classes and route-local state.</p>
  </section>
</template>
```

Observed examples:

- `client/src/views/LoginView.vue` defines local `email`, `password`, `status`, `errorMessage`, and `successMessage` refs above the template.
- `client/src/views/RegisterView.vue` keeps `confirmPassword` frontend-only and does not submit it to the API.
- `client/src/views/ProfileView.vue` uses local refs for active tab, profile form state, password form state, and status messages.
- `client/src/components/AppHeader.vue` uses `computed()` to derive visible navigation from route metadata.

---

## Props Conventions

There are no current prop-driven components in `client/src/components/`. Existing shared components (`AppHeader.vue`, `AppFooter.vue`) read from router/store modules or render static shell content.

When props are introduced later, keep them explicit and local to the component that needs them. Do not create generic prop APIs for one-off pages. In this plain JavaScript project, document required prop shapes in nearby comments or role docs until a type system is introduced.

---

## Styling Patterns

Styling is global CSS in `client/src/style.css`. Components and views reuse shared classes rather than scoped CSS blocks:

- Layout and shell: `site-header`, `site-nav`, `brand`, `app-shell`.
- Auth forms: `auth-page`, `auth-copy`, `auth-card`, `form-field`, `form-button`.
- Status messages: `notice-box`, `is-error`, `is-success`, `status-panel`.
- Profile UI: `profile-layout`, `profile-card`, `profile-tabs`, `placeholder-card`.

Do not add Tailwind, CSS modules, CSS-in-JS, or per-component styling systems without a separate dependency and design decision. For new pages, extend `style.css` with existing naming style and reuse shared button/message classes where possible.

---

## Accessibility

Current patterns to preserve:

- Use semantic form elements with `label` wrapping inputs, as in `LoginView.vue`, `RegisterView.vue`, and `ProfileView.vue`.
- Use real `button` elements for actions and `RouterLink` for navigation.
- Keep `type="button"` on non-submit buttons inside forms or form-like areas.
- Preserve `aria-label` on the brand link and `aria-label` / `role="tablist"` patterns in profile tabs.
- Disabled placeholder buttons are allowed only for unavailable features such as avatar upload.

---

## Common Mistakes

- Do not hardcode backend origins in components. Use relative `/api` calls through `client/src/api/` wrappers.
- Do not submit frontend-only fields like `confirmPassword` to backend contracts.
- Do not fake avatar upload success while the storage contract is unresolved.
- Do not request Phase 4 favorites data from the Phase 2 profile placeholder.
- Do not duplicate header navigation lists outside `client/src/router/index.js` and `AppHeader.vue`.
