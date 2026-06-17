<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { getCategories, getGames, getTags } from '../../api/games'
import BaseLoading from '../../components/common/BaseLoading.vue'
import EmptyState from '../../components/common/EmptyState.vue'
import Pagination from '../../components/common/Pagination.vue'
import GameCard from '../../components/game/GameCard.vue'
import GameFilter from '../../components/game/GameFilter.vue'

const PAGE_SIZE = 12

const route = useRoute()
const router = useRouter()

const games = ref([])
const categories = ref([])
const tags = ref([])
const total = ref(0)
const page = ref(1)
const categoryId = ref('')
const tagId = ref('')
const loading = ref(false)
const filtersLoading = ref(false)
const errorMessage = ref('')
const mockMessages = ref([])

const hasGames = computed(() => games.value.length > 0)
const mockMessage = computed(() => mockMessages.value.join(' '))
const currentQuery = computed(() => {
  const query = {}

  if (page.value > 1) {
    query.page = String(page.value)
  }

  if (categoryId.value) {
    query.categoryId = categoryId.value
  }

  if (tagId.value) {
    query.tagId = tagId.value
  }

  return query
})

function getQueryState() {
  const parsedPage = Number.parseInt(route.query.page, 10)

  return {
    page: Number.isFinite(parsedPage) && parsedPage > 0 ? parsedPage : 1,
    categoryId: route.query.categoryId ? String(route.query.categoryId) : '',
    tagId: route.query.tagId ? String(route.query.tagId) : '',
  }
}

function replaceQuery(nextState) {
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
    name: 'games',
    query,
  })
}

function appendMockMessage(message) {
  if (!mockMessages.value.includes(message)) {
    mockMessages.value = [...mockMessages.value, message]
  }
}

async function loadFilters() {
  filtersLoading.value = true

  try {
    const [categoryResult, tagResult] = await Promise.all([getCategories(), getTags()])
    categories.value = categoryResult.list
    tags.value = tagResult.list

    if (categoryResult.isMock || tagResult.isMock) {
      appendMockMessage('由于网络或服务原因，当前展示离线预览内容。')
    }
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
    const result = await getGames({
      page: queryState.page,
      pageSize: PAGE_SIZE,
      categoryId: queryState.categoryId,
      tagId: queryState.tagId,
    })

    games.value = result.list
    total.value = result.total
    page.value = result.page

    if (result.isMock) {
      appendMockMessage('由于网络或服务原因，当前展示离线预览内容。')
    }
  } catch (error) {
    games.value = []
    total.value = 0
    errorMessage.value = error instanceof Error ? error.message : '文件列表加载失败'
  } finally {
    loading.value = false
  }
}

function handleFilterChange(nextFilters) {
  replaceQuery({
    page: 1,
    categoryId: nextFilters.categoryId,
    tagId: nextFilters.tagId,
  })
}

function handleResetFilters() {
  replaceQuery({
    page: 1,
    categoryId: '',
    tagId: '',
  })
}

function handlePageChange(nextPage) {
  replaceQuery({
    page: nextPage,
    categoryId: categoryId.value,
    tagId: tagId.value,
  })
}

onMounted(() => {
  loadFilters()
  loadGames()
})

watch(
  () => route.query,
  () => {
    loadGames()
  },
)
</script>

<template>
  <section class="page-heading games-heading">
    <div class="eyebrow">所有文件</div>
    <h1>文件列表</h1>
    <p>浏览站内文件条目，按分类和标签快速筛选，进入详情页查看简介、预览图与基础信息。</p>
  </section>

  <section class="game-list-page">
    <GameFilter
      :categories="categories"
      :tags="tags"
      :category-id="categoryId"
      :tag-id="tagId"
      :disabled="loading || filtersLoading"
      @change="handleFilterChange"
      @reset="handleResetFilters"
    />

    <div v-if="mockMessage" class="notice-box is-warning">
      {{ mockMessage }}
    </div>

    <BaseLoading v-if="loading && !hasGames" text="正在加载文件列表..." />

    <div v-else-if="errorMessage" class="status-panel is-error">
      <div class="status-header">
        <span class="status-dot"></span>
        <h2>加载失败</h2>
      </div>
      <p class="status-message">{{ errorMessage }}</p>
      <button type="button" @click="loadGames">重新加载</button>
    </div>

    <EmptyState
      v-else-if="!hasGames"
      title="暂无文件"
      description="当前筛选条件下没有可展示的文件，试试切换分类或标签。"
    >
      <template #action>
        <button class="secondary-button" type="button" @click="handleResetFilters">清空筛选</button>
      </template>
    </EmptyState>

    <template v-else>
      <div class="game-grid" aria-live="polite">
        <GameCard v-for="game in games" :key="game.id" :game="game" :query="currentQuery" />
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
