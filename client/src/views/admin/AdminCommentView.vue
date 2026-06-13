<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { getAdminGames } from '../../api/admin'
import { deleteComment, getComments } from '../../api/interactions'
import BaseLoading from '../../components/common/BaseLoading.vue'
import EmptyState from '../../components/common/EmptyState.vue'
import Pagination from '../../components/common/Pagination.vue'
import { useAuthStore } from '../../stores/auth'

const PAGE_SIZE = 12

const route = useRoute()
const router = useRouter()
const { authToken } = useAuthStore()

const games = ref([])
const comments = ref([])
const total = ref(0)
const page = ref(1)
const selectedGameId = ref('')
const loadingGames = ref(false)
const loadingComments = ref(false)
const deletingId = ref(null)
const errorMessage = ref('')
const successMessage = ref('')

const selectedGame = computed(() =>
  games.value.find((game) => String(game.id) === String(selectedGameId.value)),
)

const hasComments = computed(() => comments.value.length > 0)

function getQueryState() {
  const parsedPage = Number.parseInt(route.query.page, 10)

  return {
    page: Number.isFinite(parsedPage) && parsedPage > 0 ? parsedPage : 1,
    gameId: route.query.gameId ? String(route.query.gameId) : '',
  }
}

function pushQuery(nextState) {
  const query = {}

  if (nextState.gameId) {
    query.gameId = String(nextState.gameId)
  }

  if (nextState.page > 1) {
    query.page = String(nextState.page)
  }

  router.push({
    name: 'admin-comments',
    query,
  })
}

function formatDate(value) {
  if (!value) {
    return '时间未知'
  }

  return new Date(value).toLocaleString('zh-CN')
}

async function loadGames() {
  loadingGames.value = true
  errorMessage.value = ''

  try {
    const result = await getAdminGames(
      {
        page: 1,
        pageSize: 100,
      },
      authToken.value,
    )
    games.value = result.list

    const queryState = getQueryState()
    if (!queryState.gameId && result.list.length) {
      pushQuery({
        gameId: result.list[0].id,
        page: 1,
      })
    }
  } catch (error) {
    games.value = []
    errorMessage.value = error instanceof Error ? error.message : '后台游戏列表加载失败'
  } finally {
    loadingGames.value = false
  }
}

async function loadComments() {
  const queryState = getQueryState()
  selectedGameId.value = queryState.gameId
  page.value = queryState.page
  comments.value = []
  total.value = 0

  if (!queryState.gameId) {
    return
  }

  loadingComments.value = true
  errorMessage.value = ''

  try {
    const result = await getComments(queryState.gameId, {
      page: queryState.page,
      pageSize: PAGE_SIZE,
    })
    comments.value = result.list
    total.value = result.total
    page.value = result.page
  } catch (error) {
    comments.value = []
    total.value = 0
    errorMessage.value = error instanceof Error ? error.message : '评论列表加载失败'
  } finally {
    loadingComments.value = false
  }
}

function handleGameChange(event) {
  successMessage.value = ''
  pushQuery({
    gameId: event.target.value,
    page: 1,
  })
}

function handlePageChange(nextPage) {
  pushQuery({
    gameId: selectedGameId.value,
    page: nextPage,
  })
}

async function handleDelete(comment) {
  const confirmed = window.confirm(`确认删除 ${comment.username} 的这条评论吗？`)

  if (!confirmed) {
    return
  }

  deletingId.value = comment.id
  errorMessage.value = ''
  successMessage.value = ''

  try {
    await deleteComment(comment.id, authToken.value)
    successMessage.value = '评论已删除'
    await loadComments()
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '删除评论失败'
  } finally {
    deletingId.value = null
  }
}

onMounted(loadGames)

watch(
  () => route.query,
  () => {
    loadComments()
  },
  { immediate: true },
)
</script>

<template>
  <section class="page-heading admin-heading">
    <div class="eyebrow">Admin Center</div>
    <h1>评论管理</h1>
    <p>按游戏查看评论，并删除违规内容。全站评论检索需要后端后续提供管理员评论列表接口。</p>
  </section>

  <section class="admin-page">
    <div class="admin-toolbar">
      <div>
        <h2>评论审核</h2>
        <p>{{ selectedGame ? selectedGame.title : '请选择游戏' }} · 共 {{ total }} 条</p>
      </div>
      <div class="admin-nav-actions">
        <RouterLink class="secondary-button" to="/admin/games">游戏管理</RouterLink>
      </div>
    </div>

    <section class="admin-form-card">
      <label class="form-field">
        <span>选择游戏</span>
        <select :value="selectedGameId" :disabled="loadingGames" @change="handleGameChange">
          <option value="">请选择游戏</option>
          <option v-for="game in games" :key="game.id" :value="String(game.id)">
            {{ game.title }}
          </option>
        </select>
      </label>
    </section>

    <p v-if="successMessage" class="notice-box is-success">{{ successMessage }}</p>
    <p v-if="errorMessage" class="notice-box is-error">{{ errorMessage }}</p>

    <BaseLoading v-if="loadingGames || loadingComments" text="正在加载评论管理数据..." />

    <EmptyState
      v-else-if="!selectedGameId"
      title="请选择游戏"
      description="当前后端暂未提供全站管理员评论列表，先按游戏查看和删除评论。"
    />

    <EmptyState
      v-else-if="!hasComments"
      title="暂无评论"
      description="当前游戏还没有评论。"
    />

    <template v-else>
      <div class="admin-comment-list">
        <article v-for="comment in comments" :key="comment.id" class="admin-comment-item">
          <div class="admin-comment-main">
            <div class="admin-comment-meta">
              <strong>{{ comment.username }}</strong>
              <span>{{ formatDate(comment.createdAt) }}</span>
              <span v-if="comment.parentId">回复 #{{ comment.parentId }}</span>
            </div>
            <p>{{ comment.content }}</p>
          </div>
          <div class="admin-actions">
            <RouterLink
              class="secondary-button"
              :to="{ name: 'game-detail', params: { id: comment.gameId || selectedGameId } }"
            >
              查看游戏
            </RouterLink>
            <button
              class="ghost-button danger-button"
              type="button"
              :disabled="deletingId === comment.id"
              @click="handleDelete(comment)"
            >
              {{ deletingId === comment.id ? '删除中...' : '删除评论' }}
            </button>
          </div>
        </article>
      </div>

      <Pagination
        v-if="total > PAGE_SIZE"
        :page="page"
        :page-size="PAGE_SIZE"
        :total="total"
        :disabled="loadingComments"
        @update:page="handlePageChange"
      />
    </template>
  </section>
</template>
