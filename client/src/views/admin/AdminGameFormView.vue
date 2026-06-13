<script setup>
import { computed, onMounted, reactive, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { createAdminGame, updateAdminGame } from '../../api/admin'
import { getCategories, getGameDetail, getTags } from '../../api/games'
import AdminImageUpload from '../../components/admin/AdminImageUpload.vue'
import DownloadLinkEditor from '../../components/admin/DownloadLinkEditor.vue'
import BaseLoading from '../../components/common/BaseLoading.vue'
import EmptyState from '../../components/common/EmptyState.vue'
import { useAuthStore } from '../../stores/auth'

const route = useRoute()
const router = useRouter()
const { authToken } = useAuthStore()

const form = reactive({
  title: '',
  developer: '',
  publisher: '',
  releaseDate: '',
  description: '',
  coverUrl: '',
  categoryId: '',
  tagIds: [],
  screenshots: [],
})

const categories = ref([])
const tags = ref([])
const loading = ref(false)
const saving = ref(false)
const optionsLoading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const optionNotice = ref('')

const isCreateMode = computed(() => route.name === 'admin-game-new')
const pageTitle = computed(() => (isCreateMode.value ? '新增游戏' : '编辑游戏'))
const currentGameId = computed(() => route.params.id)

function emptyScreenshot() {
  return {
    url: '',
    sortOrder: form.screenshots.length + 1,
  }
}

function resetForm() {
  form.title = ''
  form.developer = ''
  form.publisher = ''
  form.releaseDate = ''
  form.description = ''
  form.coverUrl = ''
  form.categoryId = ''
  form.tagIds = []
  form.screenshots = [emptyScreenshot()]
}

function fillForm(game) {
  form.title = game.title || ''
  form.developer = game.developer || ''
  form.publisher = game.publisher || ''
  form.releaseDate = game.releaseDate || ''
  form.description = game.description || ''
  form.coverUrl = game.coverUrl || ''
  form.categoryId = game.category?.id ? String(game.category.id) : ''
  form.tagIds = Array.isArray(game.tags) ? game.tags.map((tag) => String(tag.id)) : []
  form.screenshots = game.screenshots?.length
    ? game.screenshots.map((screenshot, index) => ({
        url: screenshot.url || '',
        sortOrder: screenshot.sortOrder || index + 1,
      }))
    : [emptyScreenshot()]
}

function toPayload() {
  return {
    title: form.title.trim(),
    developer: form.developer.trim(),
    publisher: form.publisher.trim(),
    releaseDate: form.releaseDate || null,
    description: form.description.trim(),
    coverUrl: form.coverUrl.trim() || null,
    categoryId: Number(form.categoryId),
    tagIds: form.tagIds.map((tagId) => Number(tagId)),
    screenshots: form.screenshots
      .filter((screenshot) => screenshot.url.trim())
      .map((screenshot, index) => ({
        url: screenshot.url.trim(),
        sortOrder: Number(screenshot.sortOrder) || index + 1,
      })),
  }
}

function validateForm() {
  if (!form.title.trim()) {
    return '请输入游戏标题'
  }

  if (!form.developer.trim()) {
    return '请输入开发商'
  }

  if (!form.publisher.trim()) {
    return '请输入发行商'
  }

  if (!form.categoryId) {
    return '请选择分类'
  }

  if (!form.description.trim()) {
    return '请输入游戏简介'
  }

  return ''
}

function addScreenshot() {
  form.screenshots.push(emptyScreenshot())
}

function removeScreenshot(index) {
  form.screenshots.splice(index, 1)

  if (!form.screenshots.length) {
    form.screenshots.push(emptyScreenshot())
  }
}

function applyCoverUrl(imageUrl) {
  form.coverUrl = imageUrl
}

function applyScreenshotUrl(index, imageUrl) {
  form.screenshots[index].url = imageUrl
}

async function loadOptions() {
  optionsLoading.value = true

  try {
    const [categoryResult, tagResult] = await Promise.all([getCategories(), getTags()])
    categories.value = categoryResult.list
    tags.value = tagResult.list

    if (categoryResult.isMock || tagResult.isMock) {
      optionNotice.value = '分类/标签接口暂不可用，表单选项当前来自前端 fallback。'
    }
  } catch (error) {
    optionNotice.value = error instanceof Error ? error.message : '分类和标签加载失败'
  } finally {
    optionsLoading.value = false
  }
}

async function loadGame() {
  if (isCreateMode.value) {
    resetForm()
    return
  }

  loading.value = true
  errorMessage.value = ''

  try {
    const result = await getGameDetail(route.params.id)
    fillForm(result.game)

    if (result.isMock) {
      optionNotice.value = '当前编辑数据来自前端 mock fallback，保存仍需要真实后端接口。'
    }
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '游戏详情加载失败'
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

    if (isCreateMode.value) {
      const game = await createAdminGame(payload, authToken.value)
      successMessage.value = '游戏已创建'
      router.replace({
        name: 'admin-game-edit',
        params: {
          id: game.id,
        },
      })
      return
    }

    await updateAdminGame(route.params.id, payload, authToken.value)
    successMessage.value = '游戏已保存'
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '保存游戏失败'
  } finally {
    saving.value = false
  }
}

onMounted(() => {
  loadOptions()
})

watch(
  () => route.params.id,
  () => {
    loadGame()
  },
  { immediate: true },
)
</script>

<template>
  <section class="page-heading admin-heading">
    <div class="eyebrow">Admin Center</div>
    <h1>{{ pageTitle }}</h1>
    <p>维护游戏基础资料、分类标签、截图资源和下载链接。图片上传成功后会回填 URL。</p>
  </section>

  <BaseLoading v-if="loading" text="正在加载游戏资料..." />

  <EmptyState v-else-if="errorMessage && !isCreateMode && !form.title" title="游戏资料加载失败" :description="errorMessage">
    <template #action>
      <RouterLink class="secondary-button" to="/admin/games">返回游戏管理</RouterLink>
    </template>
  </EmptyState>

  <div v-else class="admin-form-page">
    <div class="admin-form-actions">
      <div class="admin-nav-actions">
        <RouterLink class="secondary-button" to="/admin/games">返回列表</RouterLink>
        <RouterLink class="secondary-button" to="/admin/comments">评论管理</RouterLink>
      </div>
      <button class="primary-button" type="button" :disabled="saving || optionsLoading" @click="handleSubmit">
        {{ saving ? '保存中...' : '保存游戏' }}
      </button>
    </div>

    <p v-if="optionNotice" class="notice-box is-warning">{{ optionNotice }}</p>
    <p v-if="errorMessage" class="notice-box is-error">{{ errorMessage }}</p>
    <p v-if="successMessage" class="notice-box is-success">{{ successMessage }}</p>

    <section class="admin-form-card">
      <div class="form-heading">
        <span class="pill">Basic</span>
        <h2>基础信息</h2>
      </div>

      <div class="admin-form-grid">
        <label class="form-field">
          <span>游戏标题</span>
          <input v-model="form.title" type="text" autocomplete="off" />
        </label>
        <label class="form-field">
          <span>发行日期</span>
          <input v-model="form.releaseDate" type="date" />
        </label>
        <label class="form-field">
          <span>开发商</span>
          <input v-model="form.developer" type="text" autocomplete="off" />
        </label>
        <label class="form-field">
          <span>发行商</span>
          <input v-model="form.publisher" type="text" autocomplete="off" />
        </label>
      </div>

      <label class="form-field">
        <span>简介</span>
        <textarea v-model="form.description" rows="6"></textarea>
      </label>
    </section>

    <section class="admin-form-card">
      <div class="form-heading">
        <span class="pill">Media</span>
        <h2>封面与截图</h2>
      </div>

      <label class="form-field">
        <span>封面 URL</span>
        <input v-model="form.coverUrl" type="url" placeholder="/uploads/images/example.webp" />
      </label>
      <AdminImageUpload label="上传封面图片" @uploaded="applyCoverUrl" />

      <div class="screenshot-editor">
        <div class="section-heading">
          <h2>截图 URL</h2>
          <button class="secondary-button" type="button" @click="addScreenshot">新增截图</button>
        </div>

        <div v-for="(screenshot, index) in form.screenshots" :key="index" class="screenshot-editor-row">
          <label class="form-field">
            <span>URL</span>
            <input v-model="screenshot.url" type="url" placeholder="/uploads/images/screenshot.webp" />
          </label>
          <label class="form-field">
            <span>排序</span>
            <input v-model.number="screenshot.sortOrder" type="number" min="1" />
          </label>
          <AdminImageUpload
            label="上传截图"
            help="上传成功后会填入当前截图 URL。"
            @uploaded="(imageUrl) => applyScreenshotUrl(index, imageUrl)"
          />
          <button class="ghost-button danger-button" type="button" @click="removeScreenshot(index)">删除</button>
        </div>
      </div>
    </section>

    <section class="admin-form-card">
      <div class="form-heading">
        <span class="pill">Taxonomy</span>
        <h2>分类与标签</h2>
      </div>

      <div class="admin-form-grid">
        <label class="form-field">
          <span>分类</span>
          <select v-model="form.categoryId" :disabled="optionsLoading">
            <option value="">请选择分类</option>
            <option v-for="category in categories" :key="category.id" :value="String(category.id)">
              {{ category.name }}
            </option>
          </select>
        </label>
        <label class="form-field">
          <span>标签（可多选）</span>
          <select v-model="form.tagIds" multiple :disabled="optionsLoading">
            <option v-for="tag in tags" :key="tag.id" :value="String(tag.id)">
              {{ tag.name }}
            </option>
          </select>
        </label>
      </div>
    </section>

    <DownloadLinkEditor v-if="!isCreateMode && currentGameId" :game-id="currentGameId" />

    <section v-else class="admin-form-card">
      <div class="form-heading">
        <span class="pill">Downloads</span>
        <h2>下载链接管理</h2>
      </div>
      <p class="form-help">请先保存游戏条目，进入编辑页后再新增下载链接。</p>
    </section>
  </div>
</template>
