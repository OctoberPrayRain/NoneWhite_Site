<script setup>
import { ref } from 'vue'
import { RouterLink } from 'vue-router'

import { useAuthStore } from '../stores/auth'

const { registerWithCredentials } = useAuthStore()

const username = ref('')
const email = ref('')
const password = ref('')
const confirmPassword = ref('')
const status = ref('idle')
const errorMessage = ref('')
const successMessage = ref('')

function validateForm() {
  if (username.value.trim().length < 3) {
    return '用户名至少需要 3 个字符'
  }

  if (!email.value.trim()) {
    return '请输入邮箱'
  }

  if (password.value.length < 8) {
    return '密码至少需要 8 个字符'
  }

  if (password.value !== confirmPassword.value) {
    return '两次输入的密码不一致'
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
    await registerWithCredentials({
      username: username.value.trim(),
      email: email.value.trim(),
      password: password.value,
    })
    successMessage.value = '注册成功，请使用邮箱和密码登录。'
    status.value = 'success'
    password.value = ''
    confirmPassword.value = ''
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '注册失败，请稍后重试'
    status.value = 'error'
  }
}
</script>

<template>
  <section class="auth-page">
    <div class="auth-copy">
      <div class="eyebrow">Create Account</div>
      <h1>创建你的分享站账号</h1>
      <p>
        注册页只提交契约字段 <code>username</code>、<code>email</code> 和
        <code>password</code>。确认密码只在前端用于体验校验，不会发送到后端。
      </p>
    </div>

    <form class="auth-card" @submit.prevent="handleSubmit">
      <div class="form-heading">
        <span class="pill">新用户</span>
        <h2>注册账号</h2>
      </div>

      <label class="form-field">
        <span>用户名</span>
        <input v-model="username" type="text" autocomplete="username" placeholder="alice" />
      </label>

      <label class="form-field">
        <span>邮箱</span>
        <input v-model="email" type="email" autocomplete="email" placeholder="alice@example.com" />
      </label>

      <label class="form-field">
        <span>密码</span>
        <input v-model="password" type="password" autocomplete="new-password" placeholder="至少 8 位" />
      </label>

      <label class="form-field">
        <span>确认密码</span>
        <input v-model="confirmPassword" type="password" autocomplete="new-password" placeholder="再次输入密码" />
      </label>

      <p v-if="status === 'error'" class="notice-box is-error">{{ errorMessage }}</p>
      <p v-if="status === 'success'" class="notice-box is-success">
        {{ successMessage }} <RouterLink to="/login">前往登录</RouterLink>
      </p>

      <button class="primary-button form-button" type="submit" :disabled="status === 'loading'">
        {{ status === 'loading' ? '注册中...' : '注册' }}
      </button>

      <p class="form-footnote">
        已经有账号？<RouterLink to="/login">返回登录</RouterLink>
      </p>
    </form>
  </section>
</template>
