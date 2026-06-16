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
    <div class="eyebrow">系统诊断</div>
    <h1>后端连接状态</h1>
    <p>本页用于确认前端能否连接后端服务，并展示当前服务返回的基础状态。</p>
  </section>

  <section class="status-panel" :class="`is-${status}`" aria-live="polite">
    <div class="status-header">
      <span class="status-dot" aria-hidden="true"></span>
      <h2>接口状态</h2>
    </div>

    <p v-if="status === 'loading'" class="status-message">正在检查后端服务...</p>

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
      <p>{{ errorMessage.startsWith('请求失败') ? errorMessage : `请求失败：${errorMessage}` }}</p>
      <button type="button" @click="loadStatus">重新验证</button>
    </div>
  </section>
</template>
