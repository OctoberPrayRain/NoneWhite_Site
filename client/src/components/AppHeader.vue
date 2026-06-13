<script setup>
import { computed, onMounted } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { routes } from '../router'
import { useAuthStore } from '../stores/auth'

const route = useRoute()
const router = useRouter()
const { authToken, currentUser, loadCurrentUser, logout } = useAuthStore()

const isAuthenticated = computed(() => Boolean(authToken.value))
const isAdmin = computed(() => currentUser.value?.role === 'admin')

const navRoutes = computed(() =>
  routes.filter((item) => {
    if (!item.meta?.label) {
      return false
    }

    if (item.meta.guestOnly) {
      return !isAuthenticated.value
    }

    if (item.meta.requiresAdmin) {
      return isAdmin.value
    }

    if (item.meta.requiresAuth) {
      return isAuthenticated.value
    }

    return true
  }),
)

function handleLogout() {
  logout()

  if (route.meta.requiresAuth) {
    router.push('/login')
  }
}

onMounted(() => {
  if (authToken.value && !currentUser.value) {
    loadCurrentUser().catch(() => {})
  }
})
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
      <RouterLink v-for="navRoute in navRoutes" :key="navRoute.name" :to="navRoute.path">
        {{ navRoute.meta.label }}
      </RouterLink>
      <button v-if="isAuthenticated" class="nav-button" type="button" @click="handleLogout">
        退出登录<span v-if="currentUser"> · {{ currentUser.username }}</span>
      </button>
    </nav>
  </header>
</template>
