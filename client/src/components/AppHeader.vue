<script setup>
import { routes } from '../router'
</script>

<template>
  <header class="site-header">
    <RouterLink class="brand" to="/" aria-label="NoneWhite_Site 首页">
      <span class="brand-mark">N</span>
      <span>
        <strong>NoneWhite</strong>
        <small>空白分享站</small>
      </span>
    </RouterLink>

    <nav class="site-nav" aria-label="主导航">
      <RouterLink v-for="route in routes" :key="route.name" :to="route.path">
        {{ route.meta.label }}
      </RouterLink>
    </nav>
  </header>
</template>
