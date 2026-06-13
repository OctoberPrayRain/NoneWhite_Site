<script setup>
import { onMounted, ref, watch } from 'vue'

import { getPublicDownloadLinks } from '../../api/admin'
import BaseLoading from '../common/BaseLoading.vue'
import EmptyState from '../common/EmptyState.vue'

const props = defineProps({
  gameId: {
    type: [Number, String],
    required: true,
  },
})

const links = ref([])
const loading = ref(false)
const errorMessage = ref('')

async function loadDownloadLinks() {
  loading.value = true
  errorMessage.value = ''

  try {
    links.value = await getPublicDownloadLinks(props.gameId)
  } catch (error) {
    links.value = []
    errorMessage.value = error instanceof Error ? error.message : '下载链接加载失败'
  } finally {
    loading.value = false
  }
}

onMounted(loadDownloadLinks)

watch(
  () => props.gameId,
  () => {
    loadDownloadLinks()
  },
)
</script>

<template>
  <section class="detail-section download-section">
    <div class="section-heading">
      <h2>下载区域</h2>
      <span>Phase 5</span>
    </div>

    <BaseLoading v-if="loading" text="正在加载下载链接..." />

    <div v-else-if="errorMessage" class="notice-box is-error">
      {{ errorMessage }}
      <button class="secondary-button" type="button" @click="loadDownloadLinks">重新加载</button>
    </div>

    <EmptyState
      v-else-if="!links.length"
      title="暂无下载链接"
      description="管理员添加资源后，这里会展示可用下载地址。"
    />

    <div v-else class="public-download-grid">
      <article v-for="link in links" :key="link.id" class="public-download-card">
        <div class="public-download-heading">
          <span class="pill mini-pill">{{ link.platform }}</span>
          <strong>{{ link.fileSize || '文件大小待确认' }}</strong>
        </div>

        <a class="download-url" :href="link.url" target="_blank" rel="noreferrer">
          打开下载链接
        </a>

        <dl class="public-download-meta">
          <div>
            <dt>提取码</dt>
            <dd>{{ link.extractCode || '无' }}</dd>
          </div>
          <div>
            <dt>解压密码</dt>
            <dd>{{ link.password || '无' }}</dd>
          </div>
        </dl>
      </article>
    </div>
  </section>
</template>
