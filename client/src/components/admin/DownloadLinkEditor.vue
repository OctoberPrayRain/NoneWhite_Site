<script setup>
import { computed, onMounted, reactive, ref, watch } from 'vue'

import {
  createAdminDownloadLink,
  deleteAdminDownloadLink,
  getAdminDownloadLinks,
  updateAdminDownloadLink,
} from '../../api/admin'
import BaseLoading from '../common/BaseLoading.vue'
import EmptyState from '../common/EmptyState.vue'
import { useAuthStore } from '../../stores/auth'

const props = defineProps({
  gameId: {
    type: [Number, String],
    required: true,
  },
})

const { authToken } = useAuthStore()

const links = ref([])
const loading = ref(false)
const saving = ref(false)
const deletingId = ref(null)
const editingId = ref(null)
const errorMessage = ref('')
const successMessage = ref('')

const form = reactive({
  platform: '',
  url: '',
  extractCode: '',
  password: '',
  fileSize: '',
})

const isEditing = computed(() => editingId.value !== null)

function resetForm() {
  editingId.value = null
  form.platform = ''
  form.url = ''
  form.extractCode = ''
  form.password = ''
  form.fileSize = ''
}

function toPayload() {
  return {
    platform: form.platform.trim(),
    url: form.url.trim(),
    extractCode: form.extractCode.trim() || null,
    password: form.password.trim() || null,
    fileSize: form.fileSize.trim() || null,
  }
}

function validateForm() {
  if (!form.platform.trim()) {
    return '请输入下载平台'
  }

  if (!form.url.trim()) {
    return '请输入下载链接'
  }

  return ''
}

function editLink(link) {
  editingId.value = link.id
  form.platform = link.platform
  form.url = link.url
  form.extractCode = link.extractCode || ''
  form.password = link.password || ''
  form.fileSize = link.fileSize || ''
  errorMessage.value = ''
  successMessage.value = ''
}

async function loadLinks() {
  loading.value = true
  errorMessage.value = ''

  try {
    links.value = await getAdminDownloadLinks(props.gameId, authToken.value)
  } catch (error) {
    links.value = []
    errorMessage.value = error instanceof Error ? error.message : '下载链接加载失败'
  } finally {
    loading.value = false
  }
}

async function handleSubmit() {
  const validationMessage = validateForm()

  if (validationMessage) {
    errorMessage.value = validationMessage
    successMessage.value = ''
    return
  }

  saving.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = toPayload()

    if (isEditing.value) {
      await updateAdminDownloadLink(props.gameId, editingId.value, payload, authToken.value)
      successMessage.value = '下载链接已更新'
    } else {
      await createAdminDownloadLink(props.gameId, payload, authToken.value)
      successMessage.value = '下载链接已新增'
    }

    resetForm()
    await loadLinks()
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '保存下载链接失败'
  } finally {
    saving.value = false
  }
}

async function handleDelete(link) {
  const confirmed = window.confirm(`确认删除 ${link.platform} 下载链接吗？`)

  if (!confirmed) {
    return
  }

  deletingId.value = link.id
  errorMessage.value = ''
  successMessage.value = ''

  try {
    await deleteAdminDownloadLink(props.gameId, link.id, authToken.value)
    successMessage.value = '下载链接已删除'
    await loadLinks()
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '删除下载链接失败'
  } finally {
    deletingId.value = null
  }
}

onMounted(loadLinks)

watch(
  () => props.gameId,
  () => {
    resetForm()
    loadLinks()
  },
)
</script>

<template>
  <section class="admin-form-card download-link-editor">
    <div class="form-heading">
      <span class="pill">Downloads</span>
      <h2>下载链接管理</h2>
    </div>

    <p v-if="errorMessage" class="notice-box is-error">{{ errorMessage }}</p>
    <p v-if="successMessage" class="notice-box is-success">{{ successMessage }}</p>

    <form class="download-link-form" @submit.prevent="handleSubmit">
      <div class="admin-form-grid">
        <label class="form-field">
          <span>平台</span>
          <input v-model="form.platform" type="text" placeholder="百度网盘 / MEGA / 官方站点" />
        </label>
        <label class="form-field">
          <span>文件大小</span>
          <input v-model="form.fileSize" type="text" placeholder="1.2 GB" />
        </label>
      </div>
      <label class="form-field">
        <span>下载链接</span>
        <input v-model="form.url" type="url" placeholder="https://example.com/download" />
      </label>
      <div class="admin-form-grid">
        <label class="form-field">
          <span>提取码</span>
          <input v-model="form.extractCode" type="text" placeholder="可选" />
        </label>
        <label class="form-field">
          <span>解压密码</span>
          <input v-model="form.password" type="text" placeholder="可选" />
        </label>
      </div>
      <div class="download-form-actions">
        <button class="primary-button" type="submit" :disabled="saving">
          {{ saving ? '保存中...' : isEditing ? '保存修改' : '新增链接' }}
        </button>
        <button v-if="isEditing" class="secondary-button" type="button" @click="resetForm">取消编辑</button>
      </div>
    </form>

    <BaseLoading v-if="loading" text="正在加载下载链接..." />
    <EmptyState v-else-if="!links.length" title="暂无下载链接" description="新增后会同步出现在前台游戏详情下载区域。" />

    <div v-else class="download-link-list">
      <article v-for="link in links" :key="link.id" class="download-link-item">
        <div>
          <strong>{{ link.platform }}</strong>
          <a :href="link.url" target="_blank" rel="noreferrer">{{ link.url }}</a>
        </div>
        <dl>
          <div>
            <dt>文件大小</dt>
            <dd>{{ link.fileSize || '未填写' }}</dd>
          </div>
          <div>
            <dt>提取码</dt>
            <dd>{{ link.extractCode || '无' }}</dd>
          </div>
          <div>
            <dt>解压密码</dt>
            <dd>{{ link.password || '无' }}</dd>
          </div>
        </dl>
        <div class="admin-actions">
          <button class="secondary-button" type="button" @click="editLink(link)">编辑</button>
          <button
            class="ghost-button danger-button"
            type="button"
            :disabled="deletingId === link.id"
            @click="handleDelete(link)"
          >
            {{ deletingId === link.id ? '删除中...' : '删除' }}
          </button>
        </div>
      </article>
    </div>
  </section>
</template>
