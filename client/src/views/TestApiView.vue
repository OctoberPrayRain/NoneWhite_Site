<script setup>
import { onMounted, ref } from 'vue'

import { fetchTestStatus } from '../api/test'

const status = ref('loading')
const result = ref(null)
const errorMessage = ref('')

async function loadStatus() {
  status.value = 'loading'
  errorMessage.value = ''

  try {
    result.value = await fetchTestStatus()
    status.value = 'success'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '未知错误'
    status.value = 'error'
  }
}

onMounted(loadStatus)
</script>

<template>
  <section class="page-heading">
    <div class="eyebrow">GET /api/test</div>
    <h1>前后端联调验证</h1>
    <p>
      本页通过前端相对路径 <code>/api/test</code> 请求后端，Vite dev server 会把
      <code>/api</code> 自动代理到 <code>http://localhost:3000</code>。
    </p>
  </section>

  <section class="status-panel" :class="`is-${status}`" aria-live="polite">
    <div class="status-header">
      <span class="status-dot" aria-hidden="true"></span>
      <h2>接口状态</h2>
    </div>

    <p v-if="status === 'loading'" class="status-message">正在请求后端测试接口...</p>

    <div v-else-if="status === 'success' && result" class="result-grid">
      <div>
        <span>code</span>
        <strong>{{ result.code }}</strong>
      </div>
      <div>
        <span>service</span>
        <strong>{{ result.data?.service }}</strong>
      </div>
      <div>
        <span>status</span>
        <strong>{{ result.data?.status }}</strong>
      </div>
      <div class="message-row">
        <span>message</span>
        <strong>{{ result.message }}</strong>
      </div>
    </div>

    <div v-else class="error-box">
      <p>请求失败：{{ errorMessage }}</p>
      <button type="button" @click="loadStatus">重新验证</button>
    </div>
  </section>
</template>
