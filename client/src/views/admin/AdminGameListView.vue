<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { deleteAdminGame, getAdminGames } from '../../api/admin'
import { getCategories, getTags } from '../../api/games'
import BaseLoading from '../../components/common/BaseLoading.vue'
import EmptyState from '../../components/common/EmptyState.vue'
import Pagination from '../../components/common/Pagination.vue'
import GameFilter from '../../components/game/GameFilter.vue'
import { useAuthStore } from '../../stores/auth'

const PAGE_SIZE = 12

const route = useRoute()
const router = useRouter()
const { authToken } = useAuthStore()

const games = ref([])
const categories = ref([])
const tags = ref([])
const total = ref(0)
const page = ref(1)
const categoryId = ref('')
const tagId = ref('')
const loading = ref(false)
const filtersLoading = ref(false)
const deletingId = ref(null)
const errorMessage = ref('')
const successMessage = ref('')
const filterNotice = ref('')

const hasGames = computed(() => games.value.length > 0)

function getQueryState() {
  const parsedPage = Number.parseInt(route.query.page, 10)

  return {
    page: Number.isFinite(parsedPage) && parsedPage > 0 ? parsedPage : 1,
    categoryId: route.query.categoryId ? String(route.query.categoryId) : '',
    tagId: route.query.tagId ? String(route.query.tagId) : '',
  }
}

function pushQuery(nextState) {
  const query = {}

  if (nextState.page > 1) {
    query.page = String(nextState.page)
  }

  if (nextState.categoryId) {
    query.categoryId = nextState.categoryId
  }

  if (nextState.tagId) {
    query.tagId = nextState.tagId
  }

  router.push({
    name: 'admin-games',
    query,
  })
}

function formatDate(value) {
  return value || '待确认'
}

async function loadFilters() {
  filtersLoading.value = true

  try {
    const [categoryResult, tagResult] = await Promise.all([getCategories(), getTags()])
    categories.value = categoryResult.list
    tags.value = tagResult.list

    if (categoryResult.isMock || tagResult.isMock) {
      filterNotice.value = '分类/标签接口暂不可用，筛选项当前来自前端 fallback。'
    }
  } catch (error) {
    filterNotice.value = error instanceof Error ? error.message : '分类和标签加载失败'
  } finally {
    filtersLoading.value = false
  }
}

async function loadGames() {
  const queryState = getQueryState()
  page.value = queryState.page
  categoryId.value = queryState.categoryId
  tagId.value = queryState.tagId
  loading.value = true
  errorMessage.value = ''

  try {
    const result = await getAdminGames(
      {
        page: queryState.page,
        pageSize: PAGE_SIZE,
        categoryId: queryState.categoryId,
        tagId: queryState.tagId,
      },
      authToken.value,
    )

    games.value = result.list
    total.value = result.total
    page.value = result.page
  } catch (error) {
    games.value = []
    total.value = 0
    errorMessage.value = error instanceof Error ? error.message : '后台游戏列表加载失败'
  } finally {
    loading.value = false
  }
}

function handleFilterChange(nextFilters) {
  successMessage.value = ''
  pushQuery({
    page: 1,
    categoryId: nextFilters.categoryId,
    tagId: nextFilters.tagId,
  })
}

function handleResetFilters() {
  successMessage.value = ''
  pushQuery({
    page: 1,
    categoryId: '',
    tagId: '',
  })
}

function handlePageChange(nextPage) {
  pushQuery({
    page: nextPage,
    categoryId: categoryId.value,
    tagId: tagId.value,
  })
}

async function handleDeleteGame(game) {
  const confirmed = window.confirm(`确认删除《${game.title}》吗？该操作会删除游戏及其关联数据。`)

  if (!confirmed) {
    return
  }

  deletingId.value = game.id
  errorMessage.value = ''
  successMessage.value = ''

  try {
    await deleteAdminGame(game.id, authToken.value)
    successMessage.value = `《${game.title}》已删除`
    await loadGames()
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '删除游戏失败'
  } finally {
    deletingId.value = null
  }
}

onMounted(loadFilters)

watch(
  () => route.query,
  () => {
    loadGames()
  },
  { immediate: true },
)
</script>

<template>
  <section class="page-heading admin-heading">
    <div class="eyebrow">Admin Center</div>
    <h1>游戏管理</h1>
    <p>维护游戏条目、基础信息与后续资源配置。新增和编辑会调用 Phase 5 后台接口。</p>
  </section>

  <section class="admin-page">
    <div class="admin-toolbar">
      <div>
        <h2>游戏条目</h2>
        <p>共 {{ total }} 条</p>
      </div>
      <div class="admin-nav-actions">
        <RouterLink class="secondary-button" to="/admin/comments">评论管理</RouterLink>
        <RouterLink class="primary-button" to="/admin/games/new">新增游戏</RouterLink>
      </div>
    </div>

    <GameFilter
      :categories="categories"
      :tags="tags"
      :category-id="categoryId"
      :tag-id="tagId"
      :disabled="loading || filtersLoading"
      @change="handleFilterChange"
      @reset="handleResetFilters"
    />

    <p v-if="filterNotice" class="notice-box is-warning">{{ filterNotice }}</p>
    <p v-if="successMessage" class="notice-box is-success">{{ successMessage }}</p>

    <BaseLoading v-if="loading && !hasGames" text="正在加载后台游戏列表..." />

    <div v-else-if="errorMessage" class="status-panel is-error">
      <div class="status-header">
        <span class="status-dot" aria-hidden="true"></span>
        <h2>加载失败</h2>
      </div>
      <p class="status-message">{{ errorMessage }}</p>
      <button type="button" @click="loadGames">重新加载</button>
    </div>

    <EmptyState v-else-if="!hasGames" title="暂无游戏" description="当前筛选条件下没有游戏条目。">
      <template #action>
        <RouterLink class="primary-button" to="/admin/games/new">新增游戏</RouterLink>
      </template>
    </EmptyState>

    <template v-else>
      <div class="admin-table" role="table" aria-label="后台游戏列表">
        <div class="admin-table-row admin-table-head" role="row">
          <span role="columnheader">游戏</span>
          <span role="columnheader">分类</span>
          <span role="columnheader">发行日期</span>
          <span role="columnheader">数据</span>
          <span role="columnheader">操作</span>
        </div>

        <article v-for="game in games" :key="game.id" class="admin-table-row" role="row">
          <div class="admin-game-cell" role="cell">
            <strong>{{ game.title }}</strong>
            <span>{{ game.developer }} / {{ game.publisher }}</span>
          </div>
          <div role="cell">
            {{ game.category?.name || '未分类' }}
          </div>
          <div role="cell">
            {{ formatDate(game.releaseDate) }}
          </div>
          <div class="admin-game-stats" role="cell">
            <span>点赞 {{ game.likesCount }}</span>
            <span>收藏 {{ game.favoritesCount }}</span>
          </div>
          <div class="admin-actions" role="cell">
            <RouterLink class="secondary-button" :to="{ name: 'admin-game-edit', params: { id: game.id } }">
              编辑
            </RouterLink>
            <RouterLink class="secondary-button" :to="{ name: 'game-detail', params: { id: game.id } }">
              查看
            </RouterLink>
            <button
              class="ghost-button danger-button"
              type="button"
              :disabled="deletingId === game.id"
              @click="handleDeleteGame(game)"
            >
              {{ deletingId === game.id ? '删除中...' : '删除' }}
            </button>
          </div>
        </article>
      </div>

      <Pagination
        v-if="total > PAGE_SIZE"
        :page="page"
        :page-size="PAGE_SIZE"
        :total="total"
        :disabled="loading"
        @update:page="handlePageChange"
      />
    </template>
  </section>
</template>
