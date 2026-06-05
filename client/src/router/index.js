import { createRouter, createWebHistory } from 'vue-router'

import HomeView from '../views/HomeView.vue'
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
