<template>
  <div class="app-shell">
    <AppHeader />
    <main class="site-main">
      <RouterView />
    </main>
    <AppFooter />
  </div>
</template>

<script setup>
import AppFooter from './components/AppFooter.vue'
import AppHeader from './components/AppHeader.vue'
</script>
