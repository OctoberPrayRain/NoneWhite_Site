<script setup>
import { computed, ref, watch } from 'vue'

import { getMyFavorites } from '../../api/interactions'
import { useAuthStore } from '../../stores/auth'
import BaseLoading from '../common/BaseLoading.vue'
import EmptyState from '../common/EmptyState.vue'
import Pagination from '../common/Pagination.vue'
import GameCard from '../game/GameCard.vue'

const PAGE_SIZE = 12

const props = defineProps({
  active: {
    type: Boolean,
    default: false,
  },
})

const { authToken } = useAuthStore()

const favorites = ref([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const loadedOnce = ref(false)
const errorMessage = ref('')

const hasFavorites = computed(() => favorites.value.length > 0)

function toErrorMessage(error) {
  return error instanceof Error ? error.message : '收藏列表加载失败'
}

async function loadFavorites() {
  if (!authToken.value) {
    favorites.value = []
    total.value = 0
    loadedOnce.value = false
    errorMessage.value = ''
    return
  }

  loading.value = true
  errorMessage.value = ''

  try {
    const result = await getMyFavorites(
      {
        page: page.value,
        pageSize: PAGE_SIZE,
      },
      authToken.value,
    )

    favorites.value = result.list
    total.value = result.total
    page.value = result.page
    loadedOnce.value = true
  } catch (error) {
    favorites.value = []
    total.value = 0
    errorMessage.value = toErrorMessage(error)
  } finally {
    loading.value = false
  }
}

function handlePageChange(nextPage) {
  page.value = nextPage
  loadFavorites()
}

watch(
  () => props.active,
  (active) => {
    if (active && !loadedOnce.value) {
      loadFavorites()
    }
  },
  { immediate: true },
)

watch(() => authToken.value, (token) => {
  if (!token) {
    favorites.value = []
    total.value = 0
    page.value = 1
    loadedOnce.value = false
    errorMessage.value = ''
    return
  }

  if (props.active && !loadedOnce.value) {
    loadFavorites()
  }
})
</script>

<template>
  <article class="profile-card favorites-card">
    <div class="form-heading">
      <span class="pill">Favorites</span>
      <h2>收藏列表</h2>
    </div>

    <p class="form-help">这里展示您收藏的所有游戏。</p>

    <BaseLoading v-if="loading && !hasFavorites" text="正在加载收藏列表..." />

    <div v-else-if="errorMessage" class="status-panel is-error compact-panel">
      <div class="status-header">
        <span class="status-dot" aria-hidden="true"></span>
        <h2>收藏加载失败</h2>
      </div>
      <p class="status-message">{{ errorMessage }}</p>
      <button class="secondary-button" type="button" @click="loadFavorites">重新加载</button>
    </div>

    <EmptyState
      v-else-if="loadedOnce && !hasFavorites"
      title="暂时没有收藏"
      description="去游戏详情页收藏你感兴趣的作品，这里会自动展示最新列表。"
    />

    <template v-else>
      <div class="profile-favorites-grid">
        <GameCard v-for="game in favorites" :key="game.id" :game="game" />
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
  </article>
</template>
