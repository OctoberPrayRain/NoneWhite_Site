<script setup>
import { computed, onMounted, ref } from 'vue'

import {
  adminApproveGame,
  adminCreateDownloadLink,
  adminCreateGame,
  adminDeleteDownloadLink,
  adminDeleteGame,
  adminGetDownloadLinks,
  adminGetGames,
  adminGetPendingGames,
  adminUpdateDownloadLink,
  adminUpdateGame,
  uploadImage,
} from '../api/admin'
import { getCategories, getTags } from '../api/games'
import { deleteComment, getComments } from '../api/interactions'
import BaseLoading from '../components/common/BaseLoading.vue'
import EmptyState from '../components/common/EmptyState.vue'
import Pagination from '../components/common/Pagination.vue'
import { useAuthStore } from '../stores/auth'

const PAGE_SIZE = 12

const { authToken, currentUser, loadCurrentUser } = useAuthStore()

const loading = ref(false)
const savingGame = ref(false)
const uploading = ref(false)
const resourceLoading = ref(false)
const errorMessage = ref('')
const successMessage = ref('')
const games = ref([])
const gamePage = ref(1)
const gameTotal = ref(0)
const pendingGames = ref([])
const pendingLoading = ref(false)
const categories = ref([])
const tags = ref([])
const selectedGameId = ref('')
const downloadLinks = ref([])
const comments = ref([])
const editingGameId = ref(null)
const editingDownloadLinkId = ref(null)

const isAdmin = computed(() => currentUser.value?.role === 'admin')
const selectedGame = computed(() => games.value.find((game) => String(game.id) === String(selectedGameId.value)) || null)
const hasGames = computed(() => games.value.length > 0)

const emptyGameForm = () => ({
  title: '',
  developer: '',
  publisher: '',
  releaseDate: '',
  description: '',
  coverUrl: '',
  categoryId: '',
  tagIds: [],
  screenshotsText: '',
})

const emptyDownloadForm = () => ({
  platform: '',
  url: '',
  extractCode: '',
  password: '',
  fileSize: '',
})

const gameForm = ref(emptyGameForm())
const downloadForm = ref(emptyDownloadForm())

let gameRequestId = 0
let resourceRequestId = 0
let gameMutationRequestId = 0
let uploadRequestId = 0

function toErrorMessage(error, fallback) {
  return error instanceof Error ? error.message : fallback
}

function resetGameForm() {
  editingGameId.value = null
  gameForm.value = emptyGameForm()
}

function resetDownloadForm() {
  editingDownloadLinkId.value = null
  downloadForm.value = emptyDownloadForm()
}

function clearResourceState() {
  resourceRequestId += 1
  resourceLoading.value = false
  downloadLinks.value = []
  comments.value = []
  resetDownloadForm()
}

function isCurrentGameMutation(requestId, editingId, selectedId, formRef) {
  return (
    requestId === gameMutationRequestId &&
    editingGameId.value === editingId &&
    selectedGameId.value === selectedId &&
    gameForm.value === formRef
  )
}

function isCurrentUpload(requestId, editingId, selectedId, formRef) {
  return (
    requestId === uploadRequestId &&
    editingGameId.value === editingId &&
    selectedGameId.value === selectedId &&
    gameForm.value === formRef
  )
}

function isCurrentDownloadMutation(requestId, gameId, editingId, formRef) {
  return (
    requestId === resourceRequestId &&
    selectedGameId.value === gameId &&
    editingDownloadLinkId.value === editingId &&
    downloadForm.value === formRef
  )
}

function buildGamePayload() {
  return {
    title: gameForm.value.title,
    developer: gameForm.value.developer,
    publisher: gameForm.value.publisher,
    releaseDate: gameForm.value.releaseDate || null,
    description: gameForm.value.description,
    coverUrl: gameForm.value.coverUrl || null,
    categoryId: Number(gameForm.value.categoryId),
    tagIds: gameForm.value.tagIds.map((tagId) => Number(tagId)),
    screenshots: gameForm.value.screenshotsText
      .split('\n')
      .map((url) => url.trim())
      .filter(Boolean)
      .map((url, index) => ({ url, sortOrder: index })),
  }
}

function buildDownloadPayload() {
  return {
    platform: downloadForm.value.platform,
    url: downloadForm.value.url,
    extractCode: downloadForm.value.extractCode || null,
    password: downloadForm.value.password || null,
    fileSize: downloadForm.value.fileSize || null,
  }
}

async function ensureUserLoaded() {
  if (authToken.value && !currentUser.value) {
    await loadCurrentUser()
  }
}

async function loadPendingGames() {
  pendingLoading.value = true
  try {
    const result = await adminGetPendingGames({ pageSize: 50 }, authToken.value)
    pendingGames.value = result.list
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '加载待审核游戏失败')
  } finally {
    pendingLoading.value = false
  }
}

async function handleApproveGame(gameId) {
  try {
    await adminApproveGame(gameId, authToken.value)
    successMessage.value = '游戏审核通过'
    await Promise.all([loadPendingGames(), loadGames(gamePage.value)])
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '审核游戏失败')
  }
}

async function loadLookups() {
  const [categoryResult, tagResult] = await Promise.all([getCategories(), getTags()])
  categories.value = categoryResult.list
  tags.value = tagResult.list
}

async function loadGames(nextPage = gamePage.value) {
  const currentRequestId = ++gameRequestId
  loading.value = true
  errorMessage.value = ''

  try {
    const result = await adminGetGames({ page: nextPage, pageSize: PAGE_SIZE }, authToken.value)

    if (currentRequestId !== gameRequestId) {
      return
    }

    games.value = result.list
    gameTotal.value = result.total
    gamePage.value = result.page

    const previousSelectedGameId = selectedGameId.value
    const selectedGameIsVisible = result.list.some((game) => String(game.id) === previousSelectedGameId)
    const nextSelectedGameId = selectedGameIsVisible ? previousSelectedGameId : String(result.list[0]?.id ?? '')

    if (!nextSelectedGameId) {
      selectedGameId.value = ''
      clearResourceState()
      return
    }

    if (nextSelectedGameId !== previousSelectedGameId) {
      selectedGameId.value = nextSelectedGameId
      resetDownloadForm()
      await loadResources(nextSelectedGameId)
    }
  } catch (error) {
    if (currentRequestId === gameRequestId) {
      games.value = []
      gameTotal.value = 0
      gamePage.value = nextPage
      selectedGameId.value = ''
      clearResourceState()
      errorMessage.value = toErrorMessage(error, '管理员游戏列表加载失败')
    }
  } finally {
    if (currentRequestId === gameRequestId) {
      loading.value = false
    }
  }
}

async function loadResources(gameId = selectedGameId.value) {
  const currentRequestId = ++resourceRequestId
  const targetGameId = String(gameId || '')

  if (!targetGameId) {
    resourceLoading.value = false
    downloadLinks.value = []
    comments.value = []
    resetDownloadForm()
    return
  }

  resourceLoading.value = true
  errorMessage.value = ''
  downloadLinks.value = []
  comments.value = []

  try {
    const [links, commentResult] = await Promise.all([
      adminGetDownloadLinks(targetGameId, authToken.value),
      getComments(targetGameId, { page: 1, pageSize: 50 }),
    ])

    if (currentRequestId === resourceRequestId && selectedGameId.value === targetGameId) {
      downloadLinks.value = links
      comments.value = commentResult.list
    }
  } catch (error) {
    if (currentRequestId === resourceRequestId && selectedGameId.value === targetGameId) {
      downloadLinks.value = []
      comments.value = []
      errorMessage.value = toErrorMessage(error, '资源信息加载失败')
    }
  } finally {
    if (currentRequestId === resourceRequestId && selectedGameId.value === targetGameId) {
      resourceLoading.value = false
    }
  }
}

function editGame(game) {
  const nextGameId = String(game.id)
  const selectionChanged = selectedGameId.value !== nextGameId

  editingGameId.value = game.id
  selectedGameId.value = nextGameId
  gameForm.value = {
    title: game.title,
    developer: game.developer,
    publisher: game.publisher,
    releaseDate: game.releaseDate || '',
    description: game.description,
    coverUrl: game.coverUrl || '',
    categoryId: String(game.category?.id ?? ''),
    tagIds: game.tags.map((tag) => String(tag.id)),
    screenshotsText: game.screenshots.map((screenshot) => screenshot.url).filter(Boolean).join('\n'),
  }

  if (selectionChanged) {
    resetDownloadForm()
  }
  loadResources(nextGameId)
}

function selectGame(gameId) {
  const nextGameId = String(gameId)
  if (selectedGameId.value === nextGameId) {
    return
  }

  selectedGameId.value = nextGameId
  resetDownloadForm()
  loadResources(nextGameId)
}

function handleGamePageChange(nextPage) {
  loadGames(nextPage)
}

async function handleSaveGame() {
  const currentRequestId = ++gameMutationRequestId
  const targetEditingGameId = editingGameId.value
  const targetSelectedGameId = selectedGameId.value
  const targetForm = gameForm.value
  savingGame.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = buildGamePayload()
    const wasEditing = Boolean(targetEditingGameId)
    const savedGame = wasEditing
      ? await adminUpdateGame(targetEditingGameId, payload, authToken.value)
      : await adminCreateGame(payload, authToken.value)

    if (!isCurrentGameMutation(currentRequestId, targetEditingGameId, targetSelectedGameId, targetForm)) {
      return
    }

    successMessage.value = wasEditing ? '游戏已更新。' : '游戏已创建。'
    selectedGameId.value = String(savedGame.id)
    resetGameForm()
    await loadGames(wasEditing ? gamePage.value : 1)
    if (currentRequestId === gameMutationRequestId && selectedGameId.value) {
      await loadResources(selectedGameId.value)
    }
  } catch (error) {
    if (isCurrentGameMutation(currentRequestId, targetEditingGameId, targetSelectedGameId, targetForm)) {
      errorMessage.value = toErrorMessage(error, '游戏保存失败')
    }
  } finally {
    if (currentRequestId === gameMutationRequestId) {
      savingGame.value = false
    }
  }
}

async function handleDeleteGame(game) {
  if (!window.confirm(`确认删除「${game.title}」？`)) {
    return
  }

  errorMessage.value = ''
  successMessage.value = ''

  try {
    await adminDeleteGame(game.id, authToken.value)
    successMessage.value = '游戏已删除。'
    if (String(selectedGameId.value) === String(game.id)) {
      selectedGameId.value = ''
      clearResourceState()
    }
    await loadGames(games.value.length <= 1 && gamePage.value > 1 ? gamePage.value - 1 : gamePage.value)
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '游戏删除失败')
  }
}

async function handleUploadImage(event, target) {
  const file = event.target.files?.[0]
  event.target.value = ''

  if (!file) {
    return
  }

  const currentRequestId = ++uploadRequestId
  const targetEditingGameId = editingGameId.value
  const targetSelectedGameId = selectedGameId.value
  const targetForm = gameForm.value
  uploading.value = true
  errorMessage.value = ''

  try {
    const result = await uploadImage(file, authToken.value)
    if (!isCurrentUpload(currentRequestId, targetEditingGameId, targetSelectedGameId, targetForm)) {
      return
    }

    if (target === 'cover') {
      gameForm.value.coverUrl = result.imageUrl
    } else {
      gameForm.value.screenshotsText = [gameForm.value.screenshotsText, result.imageUrl]
        .filter(Boolean)
        .join('\n')
    }
    successMessage.value = '图片上传成功。'
  } catch (error) {
    if (isCurrentUpload(currentRequestId, targetEditingGameId, targetSelectedGameId, targetForm)) {
      errorMessage.value = toErrorMessage(error, '图片上传失败')
    }
  } finally {
    if (currentRequestId === uploadRequestId) {
      uploading.value = false
    }
  }
}

function editDownloadLink(link) {
  editingDownloadLinkId.value = link.id
  downloadForm.value = {
    platform: link.platform,
    url: link.url,
    extractCode: link.extractCode || '',
    password: link.password || '',
    fileSize: link.fileSize || '',
  }
}

async function handleSaveDownloadLink() {
  if (!selectedGameId.value) {
    errorMessage.value = '请先选择游戏。'
    return
  }

  const targetGameId = selectedGameId.value
  const targetEditingDownloadLinkId = editingDownloadLinkId.value
  const targetForm = downloadForm.value
  const currentRequestId = ++resourceRequestId
  resourceLoading.value = true
  errorMessage.value = ''
  successMessage.value = ''

  try {
    const payload = buildDownloadPayload()
    if (targetEditingDownloadLinkId) {
      await adminUpdateDownloadLink(targetGameId, targetEditingDownloadLinkId, payload, authToken.value)
    } else {
      await adminCreateDownloadLink(targetGameId, payload, authToken.value)
    }

    if (!isCurrentDownloadMutation(currentRequestId, targetGameId, targetEditingDownloadLinkId, targetForm)) {
      return
    }

    successMessage.value = targetEditingDownloadLinkId ? '下载链接已更新。' : '下载链接已创建。'
    resetDownloadForm()
    await loadResources(targetGameId)
  } catch (error) {
    if (isCurrentDownloadMutation(currentRequestId, targetGameId, targetEditingDownloadLinkId, targetForm)) {
      errorMessage.value = toErrorMessage(error, '下载链接保存失败')
    }
  } finally {
    if (currentRequestId === resourceRequestId && selectedGameId.value === targetGameId) {
      resourceLoading.value = false
    }
  }
}

async function handleDeleteDownloadLink(link) {
  if (!window.confirm(`确认删除「${link.platform}」下载链接？`)) {
    return
  }

  const targetGameId = selectedGameId.value

  try {
    await adminDeleteDownloadLink(targetGameId, link.id, authToken.value)
    if (selectedGameId.value === targetGameId) {
      successMessage.value = '下载链接已删除。'
      await loadResources(targetGameId)
    }
  } catch (error) {
    if (selectedGameId.value === targetGameId) {
      errorMessage.value = toErrorMessage(error, '下载链接删除失败')
    }
  }
}

async function handleDeleteComment(comment) {
  if (!window.confirm(`确认删除 ${comment.username} 的评论？`)) {
    return
  }

  const targetGameId = selectedGameId.value

  try {
    await deleteComment(comment.id, authToken.value)
    if (selectedGameId.value === targetGameId) {
      successMessage.value = '评论已删除。'
      await loadResources(targetGameId)
    }
  } catch (error) {
    if (selectedGameId.value === targetGameId) {
      errorMessage.value = toErrorMessage(error, '评论删除失败')
    }
  }
}

async function bootstrap() {
  loading.value = true
  errorMessage.value = ''

  try {
    await ensureUserLoaded()
    if (!isAdmin.value) {
      return
    }
    await loadLookups()
    await Promise.all([loadGames(), loadPendingGames()])
  } catch (error) {
    errorMessage.value = toErrorMessage(error, '管理后台初始化失败')
  } finally {
    loading.value = false
  }
}

onMounted(bootstrap)
</script>

<template>
  <section class="page-heading admin-heading">
    <div class="eyebrow">Admin Resources</div>
    <h1>管理后台</h1>
    <p>管理游戏条目、图片资源、下载链接和单个游戏下的评论内容。</p>
  </section>

  <section class="admin-page">
    <BaseLoading v-if="loading" text="正在加载管理后台..." />

    <EmptyState
      v-else-if="!isAdmin"
      title="没有管理员权限"
      description="该页面仅限管理员账号访问。"
    />

    <template v-else>
      <div v-if="errorMessage" class="notice-box is-error">{{ errorMessage }}</div>
      <div v-if="successMessage" class="notice-box is-success">{{ successMessage }}</div>

      <section class="admin-grid">
        <article class="admin-panel admin-panel--form">
          <div class="section-heading">
            <h2>{{ editingGameId ? '编辑游戏' : '创建游戏' }}</h2>
            <span>游戏条目维护</span>
          </div>

          <form class="admin-form" @submit.prevent="handleSaveGame">
            <label class="form-field">
              <span>标题</span>
              <input v-model="gameForm.title" required :disabled="savingGame || uploading" />
            </label>
            <label class="form-field">
              <span>开发商</span>
              <input v-model="gameForm.developer" required :disabled="savingGame || uploading" />
            </label>
            <label class="form-field">
              <span>发行商</span>
              <input v-model="gameForm.publisher" required :disabled="savingGame || uploading" />
            </label>
            <label class="form-field">
              <span>发行日期</span>
              <input v-model="gameForm.releaseDate" type="date" :disabled="savingGame || uploading" />
            </label>
            <label class="form-field admin-form__wide">
              <span>简介</span>
              <textarea v-model="gameForm.description" required rows="4" :disabled="savingGame || uploading"></textarea>
            </label>
            <label class="form-field">
              <span>分类</span>
              <select v-model="gameForm.categoryId" required :disabled="savingGame || uploading">
                <option value="" disabled>选择分类</option>
                <option v-for="category in categories" :key="category.id" :value="String(category.id)">
                  {{ category.name }}
                </option>
              </select>
            </label>
            <div class="form-field admin-upload-field">
              <span>封面 URL</span>
              <input v-model="gameForm.coverUrl" placeholder="/uploads/images/..." :disabled="savingGame || uploading" />
              <label class="secondary-button file-button">
                上传封面
                <input type="file" accept="image/png,image/jpeg,image/webp" :disabled="savingGame || uploading" @change="handleUploadImage($event, 'cover')" />
              </label>
            </div>
            <fieldset class="admin-tags admin-form__wide">
              <legend>标签</legend>
              <label v-for="tag in tags" :key="tag.id">
                <input v-model="gameForm.tagIds" type="checkbox" :value="String(tag.id)" :disabled="savingGame || uploading" />
                {{ tag.name }}
              </label>
            </fieldset>
            <label class="form-field admin-form__wide">
              <span>截图 URL（一行一个）</span>
              <textarea v-model="gameForm.screenshotsText" rows="4" placeholder="/uploads/images/shot.png" :disabled="savingGame || uploading"></textarea>
            </label>
            <div class="admin-form__wide admin-actions">
              <label class="secondary-button file-button">
                上传截图
                <input type="file" accept="image/png,image/jpeg,image/webp" :disabled="savingGame || uploading" @change="handleUploadImage($event, 'screenshot')" />
              </label>
              <button class="primary-button" type="submit" :disabled="savingGame || uploading">
                {{ savingGame ? '保存中...' : editingGameId ? '保存修改' : '创建游戏' }}
              </button>
              <button class="ghost-button" type="button" :disabled="savingGame || uploading" @click="resetGameForm">清空</button>
            </div>
          </form>
        </article>

        <article class="admin-panel">
          <div class="section-heading">
            <h2>游戏列表</h2>
            <span>{{ gameTotal }} 条</span>
          </div>

          <EmptyState v-if="!hasGames" title="暂无游戏" description="创建游戏后可继续管理下载链接和评论。" />
          <div v-else class="admin-game-list">
            <div
              v-for="game in games"
              :key="game.id"
              class="admin-game-row"
              :class="{ active: String(game.id) === String(selectedGameId) }"
            >
              <button class="admin-game-select" type="button" :disabled="savingGame || uploading" @click="selectGame(game.id)">
                <strong>{{ game.title }}</strong>
                <small>{{ game.developer }} · {{ game.category?.name || '未分类' }}</small>
              </button>
              <span class="admin-row-actions">
                <button type="button" :disabled="savingGame || uploading" @click.stop="editGame(game)">编辑</button>
                <button type="button" :disabled="savingGame || uploading" @click.stop="handleDeleteGame(game)">删除</button>
              </span>
            </div>
          </div>
          <Pagination
            v-if="gameTotal > PAGE_SIZE"
            :page="gamePage"
            :page-size="PAGE_SIZE"
            :total="gameTotal"
            :disabled="loading || savingGame || uploading"
            @update:page="handleGamePageChange"
          />
        </article>
      </section>

      <section class="admin-grid admin-grid--pending">
        <article class="admin-panel admin-panel--pending admin-form__wide">
          <div class="section-heading">
            <h2>待审核</h2>
            <span>{{ pendingGames.length }} 款待处理</span>
          </div>

          <BaseLoading v-if="pendingLoading" text="正在加载待审核游戏..." />
          <EmptyState v-else-if="pendingGames.length === 0" title="暂无待审核" description="当前没有等待审核的游戏提交。" />
          <div v-else class="admin-game-list">
            <div
              v-for="game in pendingGames"
              :key="game.id"
              class="admin-game-row"
            >
              <div class="admin-game-select admin-game-select--static">
                <strong>{{ game.title }}</strong>
                <small>{{ game.developer }} · {{ game.category?.name || '未分类' }} · {{ game.approvalStatus || 'pending' }}</small>
              </div>
              <span class="admin-row-actions">
                <button type="button" class="primary-button admin-approve-button" :disabled="savingGame || uploading" @click.stop="handleApproveGame(game.id)">批准</button>
              </span>
            </div>
          </div>
        </article>
      </section>

      <section class="admin-grid admin-grid--resources">
        <article class="admin-panel">
          <div class="section-heading">
            <h2>下载链接</h2>
            <span>{{ selectedGame?.title || '未选择游戏' }}</span>
          </div>

          <form class="download-form" @submit.prevent="handleSaveDownloadLink">
            <label class="form-field">
              <span>平台</span>
              <input v-model="downloadForm.platform" required placeholder="Baidu Netdisk" :disabled="resourceLoading || !selectedGameId" />
            </label>
            <label class="form-field">
              <span>URL</span>
              <input v-model="downloadForm.url" required placeholder="https://example.invalid/share" :disabled="resourceLoading || !selectedGameId" />
            </label>
            <label class="form-field">
              <span>提取码</span>
              <input v-model="downloadForm.extractCode" :disabled="resourceLoading || !selectedGameId" />
            </label>
            <label class="form-field">
              <span>密码</span>
              <input v-model="downloadForm.password" :disabled="resourceLoading || !selectedGameId" />
            </label>
            <label class="form-field">
              <span>文件大小</span>
              <input v-model="downloadForm.fileSize" placeholder="1.2 GiB" :disabled="resourceLoading || !selectedGameId" />
            </label>
            <div class="admin-actions">
              <button class="primary-button" type="submit" :disabled="resourceLoading || !selectedGameId">
                {{ editingDownloadLinkId ? '更新链接' : '新增链接' }}
              </button>
              <button class="ghost-button" type="button" :disabled="resourceLoading" @click="resetDownloadForm">清空</button>
            </div>
          </form>

          <div class="admin-resource-list">
            <p v-if="resourceLoading">正在加载资源...</p>
            <p v-else-if="downloadLinks.length === 0" class="muted-text">当前游戏暂无下载链接。</p>
            <div v-for="link in downloadLinks" v-else :key="link.id" class="admin-resource-card">
              <strong>{{ link.platform }}</strong>
              <a :href="link.url" target="_blank" rel="noreferrer">{{ link.url }}</a>
              <span>提取码：{{ link.extractCode || '无' }} · 密码：{{ link.password || '无' }} · {{ link.fileSize || '未标注大小' }}</span>
              <div class="admin-actions">
                <button class="secondary-button" type="button" :disabled="resourceLoading" @click="editDownloadLink(link)">编辑</button>
                <button class="ghost-button" type="button" :disabled="resourceLoading" @click="handleDeleteDownloadLink(link)">删除</button>
              </div>
            </div>
          </div>
        </article>

        <article class="admin-panel">
          <div class="section-heading">
            <h2>评论管理</h2>
            <span>按游戏查看</span>
          </div>

          <div class="admin-resource-list">
            <p v-if="resourceLoading">正在加载评论...</p>
            <p v-else-if="comments.length === 0" class="muted-text">当前游戏暂无评论。</p>
            <div v-for="comment in comments" v-else :key="comment.id" class="admin-resource-card comment-moderation-card">
              <strong>{{ comment.username }}</strong>
              <p>{{ comment.content }}</p>
              <small>{{ comment.createdAt }}</small>
              <button class="ghost-button" type="button" :disabled="resourceLoading" @click="handleDeleteComment(comment)">删除评论</button>
            </div>
          </div>
        </article>
      </section>
    </template>
  </section>
</template>
