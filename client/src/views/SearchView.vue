<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { getGames } from '../api/games'
import BaseLoading from '../components/common/BaseLoading.vue'
import EmptyState from '../components/common/EmptyState.vue'
import Pagination from '../components/common/Pagination.vue'
import GameCard from '../components/game/GameCard.vue'

const PAGE_SIZE = 12

const route = useRoute()
const router = useRouter()

const keywordInput = ref('')
const keyword = ref('')
const games = ref([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const errorMessage = ref('')
const mockMessage = ref('')
const searched = ref(false)

const hasGames = computed(() => games.value.length > 0)
const currentQuery = computed(() => {
  const query = {}

  if (keyword.value) {
    query.keyword = keyword.value
  }

  if (page.value > 1) {
    query.page = String(page.value)
  }

  return query
})

function readQuery() {
  const parsedPage = Number.parseInt(route.query.page, 10)
  return {
    keyword: String(route.query.keyword ?? '').trim(),
    page: Number.isFinite(parsedPage) && parsedPage > 0 ? parsedPage : 1,
  }
}

function pushSearch(nextKeyword, nextPage = 1) {
  const query = {}
  const trimmedKeyword = nextKeyword.trim()

  if (trimmedKeyword) {
    query.keyword = trimmedKeyword
  }

  if (nextPage > 1) {
    query.page = String(nextPage)
  }

  router.push({ name: 'search', query })
}

let searchRequestId = 0

function isCurrentSearchRequest(requestId, queryState) {
  const latestQueryState = readQuery()
  return (
    requestId === searchRequestId &&
    latestQueryState.keyword === queryState.keyword &&
    latestQueryState.page === queryState.page
  )
}

async function loadSearchResults() {
  const currentReqId = ++searchRequestId
  const queryState = readQuery()

  keyword.value = queryState.keyword
  keywordInput.value = queryState.keyword
  page.value = queryState.page
  errorMessage.value = ''
  mockMessage.value = ''

  if (!queryState.keyword) {
    searched.value = false
    games.value = []
    total.value = 0
    loading.value = false
    return
  }

  searched.value = true
  loading.value = true

  try {
    const result = await getGames({
      keyword: queryState.keyword,
      page: queryState.page,
      pageSize: PAGE_SIZE,
    })

    if (isCurrentSearchRequest(currentReqId, queryState)) {
      games.value = result.list
      total.value = result.total
      page.value = result.page

      if (result.isMock) {
        mockMessage.value = '由于网络或服务原因，当前为您展示离线预览搜索结果。'
      }
    }
  } catch (error) {
    if (isCurrentSearchRequest(currentReqId, queryState)) {
      games.value = []
      total.value = 0
      errorMessage.value = error instanceof Error ? error.message : '搜索失败'
    }
  } finally {
    if (isCurrentSearchRequest(currentReqId, queryState)) {
      loading.value = false
    }
  }
}

function handleSubmit() {
  pushSearch(keywordInput.value, 1)
}

function handlePageChange(nextPage) {
  pushSearch(keyword.value, nextPage)
}

onMounted(loadSearchResults)

watch(
  () => [route.query.keyword, route.query.page],
  () => {
    loadSearchResults()
  },
)
</script>

<template>
  <section class="page-heading search-heading">
    <div class="eyebrow">全站搜索</div>
    <h1>搜索文件</h1>
    <p>通过标题、提供方、发布方或标签搜索您感兴趣的文件。</p>
  </section>

  <section class="search-page">
    <form class="search-panel" @submit.prevent="handleSubmit">
      <label class="form-field">
        <span>关键词</span>
        <input v-model="keywordInput" type="search" placeholder="例如：模板 / NoneWhite / 文档" />
      </label>
      <button class="primary-button" type="submit" :disabled="loading">搜索</button>
    </form>

    <div v-if="mockMessage" class="notice-box is-warning">
      {{ mockMessage }}
    </div>

    <BaseLoading v-if="loading" text="正在搜索文件..." />

    <div v-else-if="errorMessage" class="status-panel is-error">
      <div class="status-header">
        <span class="status-dot"></span>
        <h2>搜索失败</h2>
      </div>
      <p class="status-message">{{ errorMessage }}</p>
      <button type="button" @click="loadSearchResults">重新搜索</button>
    </div>

    <EmptyState
      v-else-if="!searched"
      title="输入关键词开始搜索"
      description="输入文件名称、提供方、发布方或标签，快速找到感兴趣的作品。"
    />

    <EmptyState
      v-else-if="!hasGames"
      title="没有匹配结果"
      :description="`没有找到与「${keyword}」相关的文件。`"
    />

    <template v-else>
      <div class="search-summary">
        <strong>{{ total }}</strong>
        <span>条结果匹配「{{ keyword }}」</span>
      </div>
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
