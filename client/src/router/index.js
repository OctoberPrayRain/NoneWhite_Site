import { createRouter, createWebHistory } from 'vue-router'

import AdminCommentView from '../views/admin/AdminCommentView.vue'
import AdminGameFormView from '../views/admin/AdminGameFormView.vue'
import AdminGameListView from '../views/admin/AdminGameListView.vue'
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
  {
    path: '/admin/games',
    name: 'admin-games',
    component: AdminGameListView,
    meta: {
      label: '管理后台',
      requiresAuth: true,
      requiresAdmin: true,
    },
  },
  {
    path: '/admin/games/new',
    name: 'admin-game-new',
    component: AdminGameFormView,
    meta: {
      requiresAuth: true,
      requiresAdmin: true,
    },
  },
  {
    path: '/admin/games/:id/edit',
    name: 'admin-game-edit',
    component: AdminGameFormView,
    meta: {
      requiresAuth: true,
      requiresAdmin: true,
    },
  },
  {
    path: '/admin/comments',
    name: 'admin-comments',
    component: AdminCommentView,
    meta: {
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

  if (to.meta.requiresAdmin) {
    if (!currentUser.value) {
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

    if (currentUser.value?.role !== 'admin') {
      return '/profile'
    }
  }

  if (to.meta.guestOnly && isAuthenticated) {
    return '/profile'
  }

  return true
})

export { routes }
export default router
