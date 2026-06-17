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
    },
  },
  {
    path: '/files',
    name: 'games',
    component: GameListView,
    meta: {
      label: '文件列表',
      requiresAuth: true,
    },
  },
  {
    path: '/files/:id',
    name: 'game-detail',
    component: GameDetailView,
    meta: {
      requiresAuth: true,
    },
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
  {
    path: '/submit-file',
    name: 'submit-game',
    component: () => import('../views/SubmitGameView.vue'),
    meta: {
      label: '提交文件',
      requiresAuth: true,
    },
  },
  {
    path: '/search',
    name: 'search',
    component: () => import('../views/SearchView.vue'),
    meta: {
      label: '搜索',
      requiresAuth: true,
    },
  },
  {
    path: '/admin',
    name: 'admin',
    component: () => import('../views/AdminConsoleView.vue'),
    meta: {
      label: '管理后台',
      requiresAuth: true,
      requiresAdmin: true,
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

router.beforeEach(async (to) => {
  const { authToken, currentUser, loadCurrentUser } = useAuthStore()
  const isAuthenticated = Boolean(authToken.value)

  if (to.meta.requiresAuth && !isAuthenticated) {
    return {
      path: '/login',
      query: {
        redirect: to.fullPath,
      },
    }
  }

  if (to.meta.requiresAdmin && isAuthenticated && !currentUser.value) {
    try {
      await loadCurrentUser()
    } catch {
      return {
        path: '/login',
        query: {
          redirect: to.fullPath,
        },
      }
    }
  }

  if (to.meta.requiresAdmin && currentUser.value?.role !== 'admin') {
    return '/'
  }

  if (to.meta.guestOnly && isAuthenticated) {
    return '/profile'
  }

  return true
})

export { routes }
export default router
