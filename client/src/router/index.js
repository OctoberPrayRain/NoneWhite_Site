import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '../views/HomeView.vue'
import GameDetailView from '../views/game/GameDetailView.vue'
import GameListView from '../views/game/GameListView.vue'
import LoginView from '../views/LoginView.vue'
import ProfileView from '../views/ProfileView.vue'
import RegisterView from '../views/RegisterView.vue'
import { useAuthStore } from '../stores/auth'
import TestApiView from '../views/TestApiView.vue'

const routes = [
  {
    path: '/',
    name: 'home',
    component: HomeView,
    meta: {
      label: '首页',
    },
  },
  {
    path: '/test-api',
    name: 'test-api',
    component: TestApiView,
    meta: {
      label: '联调验证',
    },
  },
  {
    path: '/games',
    name: 'games',
    component: GameListView,
    meta: {
      label: '游戏列表',
    },
  },
  {
    path: '/games/:id',
    name: 'game-detail',
    component: GameDetailView,
  },
  {
    path: '/login',
    name: 'login',
    component: LoginView,
    meta: {
      label: '登录',
      guestOnly: true,
    },
  },
  {
    path: '/register',
    name: 'register',
    component: RegisterView,
    meta: {
      label: '注册',
      guestOnly: true,
    },
  },
  {
    path: '/profile',
    name: 'profile',
    component: ProfileView,
    meta: {
      label: '个人中心',
      requiresAuth: true,
    },
  },
]

const router = createRouter({
  history: createWebHistory(),
  routes,
  scrollBehavior() {
    return { top: 0 }
  },
})

router.beforeEach((to) => {
  const { authToken } = useAuthStore()
  const isAuthenticated = Boolean(authToken.value)

  if (to.meta.requiresAuth && !isAuthenticated) {
    return {
      path: '/login',
      query: {
        redirect: to.fullPath,
      },
    }
  }

  if (to.meta.guestOnly && isAuthenticated) {
    return '/profile'
  }

  return true
})

export { routes }
export default router
