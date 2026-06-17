<script setup>
import { onMounted, ref } from 'vue'

import { getCategories, getTags, submitGameSubmission, uploadUserImage, uploadUserResource, formatFileSize } from '../api/games'
import BaseLoading from '../components/common/BaseLoading.vue'
import { useAuthStore } from '../stores/auth'

const { authToken, currentUser, loadCurrentUser } = useAuthStore()

const loading = ref(false)
const submitting = ref(false)
const uploading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')

const categories = ref([])
const tags = ref([])

const emptyForm = () => ({
  title: '',
  developer: '',
  publisher: '',
  releaseDate: '',
  description: '',
  coverUrl: '',
  categoryId: '',
  tagIds: [],
  screenshotsText: '',
  downloadLinks: [],
})

const form = ref(emptyForm())

function resetForm() {
  form.value = emptyForm()
}

function toErrorMessage(error, fallback) {
  return error instanceof Error ? error.message : fallback
}

function buildPayload() {
  return {
    title: form.value.title,
    developer: form.value.developer,
    publisher: form.value.publisher,
    releaseDate: form.value.releaseDate || null,
    description: form.value.description,
    coverUrl: form.value.coverUrl || null,
    categoryId: Number(form.value.categoryId),
    tagIds: form.value.tagIds.map((tagId) => Number(tagId)),
    screenshots: form.value.screenshotsText
      .split('\n')
      .map((url) => url.trim())
      .filter(Boolean)
      .map((url, index) => ({ url, sortOrder: index })),
    downloadLinks: form.value.downloadLinks.map((link) => ({
      platform: link.platform || '资源文件',
      url: link.url,
      fileSize: link.fileSize || '',
    })),
  }
}

async function ensureUserLoaded() {
  if (authToken.value && !currentUser.value) {
    await loadCurrentUser()
  }
}

async function loadLookups() {
  const [categoryResult, tagResult] = await Promise.all([getCategories(), getTags()])
  categories.value = categoryResult.list
  tags.value = tagResult.list
}

async function handleUploadImage(event, target) {
  const file = event.target.files?.[0]
  event.target.value = ''
  if (!file) return

  uploading.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const result = await uploadUserImage(file, authToken.value)
    if (target === 'cover') {
      form.value.coverUrl = result.imageUrl
    } else {
      form.value.screenshotsText = [form.value.screenshotsText, result.imageUrl]
        .filter(Boolean)
        .join('\n')
    }
    successMessage.value = '图片上传成功。'
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '图片上传失败')
  } finally {
    uploading.value = false
  }
}

async function handleUploadResource(event) {
  const file = event.target.files?.[0]
  event.target.value = ''
  if (!file) return

  uploading.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const result = await uploadUserResource(file, authToken.value)
    form.value.downloadLinks.push({
      platform: '资源文件',
      url: result.resourceUrl,
      fileName: result.fileName || file.name || '已上传资源',
      fileSize: result.fileSize ? formatFileSize(result.fileSize) : '',
    })
    successMessage.value = '资源文件上传成功。'
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '资源文件上传失败')
  } finally {
    uploading.value = false
  }
}

async function handleSubmit() {
  submitting.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = buildPayload()
    await submitGameSubmission(payload, authToken.value)
    successMessage.value = '文件提交成功，等待管理员审核。'
    resetForm()
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '文件提交失败，请稍后重试。')
  } finally {
    submitting.value = false
  }
}

async function bootstrap() {
  loading.value = true
  errorMessage.value = ''
  try {
    await ensureUserLoaded()
    await loadLookups()
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '加载数据失败')
  } finally {
    loading.value = false
  }
}

onMounted(bootstrap)
</script>

<template>
  <section class="page-heading">
    <div class="eyebrow">Submit File</div>
    <h1>提交文件</h1>
    <p>在此提交新的文件。提交后将进入待审核状态，管理员审核通过后将展示在文件列表中。</p>
  </section>

  <section class="admin-page">
    <BaseLoading v-if="loading" text="正在加载..." />
    <template v-else>
      <div v-if="errorMessage" class="notice-box is-error">{{ errorMessage }}</div>
      <div v-if="successMessage" class="notice-box is-success">{{ successMessage }}</div>

      <article class="admin-panel admin-panel--form">
        <div class="section-heading">
          <h2>填写文件信息</h2>
        </div>

        <form class="admin-form" @submit.prevent="handleSubmit">
          <label class="form-field">
            <span>标题</span>
            <input v-model="form.title" required :disabled="submitting" />
          </label>
          <label class="form-field">
            <span>提供方</span>
            <input v-model="form.developer" required :disabled="submitting" />
          </label>
          <label class="form-field">
            <span>发布方</span>
            <input v-model="form.publisher" required :disabled="submitting" />
          </label>
          <label class="form-field">
            <span>发布日期</span>
            <input v-model="form.releaseDate" type="date" :disabled="submitting" />
          </label>
          <label class="form-field admin-form__wide">
            <span>简介</span>
            <textarea v-model="form.description" required rows="4" :disabled="submitting"></textarea>
          </label>
          <label class="form-field">
            <span>分类</span>
            <select v-model="form.categoryId" required :disabled="submitting">
              <option value="" disabled>选择分类</option>
              <option v-for="category in categories" :key="category.id" :value="String(category.id)">
                {{ category.name }}
              </option>
            </select>
          </label>
          <div class="form-field admin-upload-field">
            <span>封面 URL</span>
            <input v-model="form.coverUrl" placeholder="https://..." :disabled="submitting || uploading" />
            <label class="secondary-button file-button">
              上传封面
              <input type="file" accept="image/png,image/jpeg,image/webp" :disabled="submitting || uploading" @change="handleUploadImage($event, 'cover')" />
            </label>
          </div>
          <fieldset class="admin-tags admin-form__wide">
            <legend>标签</legend>
            <label v-for="tag in tags" :key="tag.id">
              <input v-model="form.tagIds" type="checkbox" :value="String(tag.id)" :disabled="submitting" />
              {{ tag.name }}
            </label>
          </fieldset>
          <div class="form-field admin-form__wide admin-upload-field">
            <span>预览图 URL（一行一个）</span>
            <textarea v-model="form.screenshotsText" rows="4" placeholder="https://..." :disabled="submitting || uploading"></textarea>
            <div class="admin-actions">
              <label class="secondary-button file-button">
                上传预览图
                <input type="file" accept="image/png,image/jpeg,image/webp" :disabled="submitting || uploading" @change="handleUploadImage($event, 'screenshot')" />
              </label>
            </div>
          </div>

          <div class="form-field admin-form__wide">
            <span>资源文件</span>
            <div v-for="(link, index) in form.downloadLinks" :key="index" class="admin-resource-card">
              <strong>{{ link.platform }}</strong>
              <span class="muted-text">{{ link.fileName }}</span>
              <span>{{ link.fileSize || '未知大小' }}</span>
              <div class="admin-actions">
                <button type="button" class="ghost-button" @click="form.downloadLinks.splice(index, 1)">删除</button>
              </div>
            </div>
            <div class="admin-actions">
              <label class="secondary-button file-button">
                上传资源
                <input type="file" :disabled="submitting || uploading" @change="handleUploadResource($event)" />
              </label>
            </div>
          </div>

          <div class="admin-form__wide admin-actions">
            <button class="primary-button" type="submit" :disabled="submitting || uploading">
              {{ submitting ? '提交中...' : '提交审核' }}
            </button>
            <button class="ghost-button" type="button" :disabled="submitting || uploading" @click="resetForm">清空</button>
          </div>
        </form>
      </article>
    </template>
  </section>
</template>
