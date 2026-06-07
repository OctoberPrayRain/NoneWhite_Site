import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '../views/HomeView.vue'
import LoginView from '../views/LoginView.vue'
import ProfileView from '../views/ProfileView.vue'
import RegisterView from '../views/RegisterView.vue'
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

export { routes }
export default router
