<script setup>
import { ref } from 'vue'
import { RouterLink, useRoute, useRouter } from 'vue-router'

import { useAuthStore } from '../stores/auth'

const route = useRoute()
const router = useRouter()
const { authToken, loginWithCredentials } = useAuthStore()

const email = ref('')
const password = ref('')
const status = ref('idle')
const errorMessage = ref('')
const successMessage = ref('')

function validateForm() {
  if (!email.value.trim()) {
    return '请输入邮箱'
  }

  if (!password.value) {
    return '请输入密码'
  }

  return ''
}

async function handleSubmit() {
  const validationMessage = validateForm()

  if (validationMessage) {
    errorMessage.value = validationMessage
    status.value = 'error'
    return
  }

  status.value = 'loading'
  errorMessage.value = ''
  successMessage.value = ''

  try {
    await loginWithCredentials({
      email: email.value.trim(),
      password: password.value,
    })
    successMessage.value = '登录成功，正在前往个人中心...'
    status.value = 'success'
    await router.push(typeof route.query.redirect === 'string' ? route.query.redirect : '/profile')
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '登录失败，请稍后重试'
    status.value = 'error'
  }
}
</script>

<template>
  <section class="auth-page">
    <div class="auth-copy">
      <div class="eyebrow">Phase 2 · User System</div>
      <h1>登录空白分享站</h1>
      <p>
        使用后端 <code>POST /api/auth/login</code> 契约登录。登录成功后，前端会保存
        <code>nonewhite_auth_token</code> 并跳转到个人中心。
      </p>
    </div>

    <form class="auth-card" @submit.prevent="handleSubmit">
      <div class="form-heading">
        <span class="pill">已有账号</span>
        <h2>欢迎回来</h2>
      </div>

      <p v-if="authToken" class="notice-box is-success">
        当前已保存登录状态，可以直接前往 <RouterLink to="/profile">个人中心</RouterLink>。
      </p>

      <label class="form-field">
        <span>邮箱</span>
        <input v-model="email" type="email" autocomplete="email" placeholder="alice@example.com" />
      </label>

      <label class="form-field">
        <span>密码</span>
        <input v-model="password" type="password" autocomplete="current-password" placeholder="至少 8 位" />
      </label>

      <p v-if="status === 'error'" class="notice-box is-error">{{ errorMessage }}</p>
      <p v-if="status === 'success'" class="notice-box is-success">{{ successMessage }}</p>

      <button class="primary-button form-button" type="submit" :disabled="status === 'loading'">
        {{ status === 'loading' ? '登录中...' : '登录' }}
      </button>

      <p class="form-footnote">
        还没有账号？<RouterLink to="/register">去注册</RouterLink>
      </p>
    </form>
  </section>
</template>
