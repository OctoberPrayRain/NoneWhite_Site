<script setup>
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

import { getGameDetail } from '../../api/games'
import BaseLoading from '../../components/common/BaseLoading.vue'
import EmptyState from '../../components/common/EmptyState.vue'
import CommentSection from '../../components/game/CommentSection.vue'
import FavoriteButton from '../../components/game/FavoriteButton.vue'
import LikeButton from '../../components/game/LikeButton.vue'
import ScreenshotCarousel from '../../components/game/ScreenshotCarousel.vue'
import { useAuthStore } from '../../stores/auth'

const route = useRoute()

const { authToken } = useAuthStore()

const game = ref(null)
const loading = ref(false)
const errorMessage = ref('')
const mockMessage = ref('')
const coverFailed = ref(false)

const backTarget = computed(() => ({
  name: 'games',
  query: route.query,
}))

const hasCover = computed(() => game.value?.coverUrl && !coverFailed.value)
const isAuthenticated = computed(() => Boolean(authToken.value))

function handleLikeUpdated(result) {
  if (!game.value) {
    return
  }

  game.value = {
    ...game.value,
    likesCount: result.likesCount,
  }
}

function handleFavoriteUpdated(result) {
  if (!game.value) {
    return
  }

  game.value = {
    ...game.value,
    favoritesCount: result.favoritesCount,
  }
}

async function loadGameDetail() {
  loading.value = true
  errorMessage.value = ''
  mockMessage.value = ''
  coverFailed.value = false
  game.value = null

  try {
    const result = await getGameDetail(route.params.id)
    game.value = result.game

    if (result.isMock) {
      mockMessage.value = '游戏详情接口暂不可用，当前详情来自前端 mock fallback。'
    }
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '游戏详情加载失败'
  } finally {
    loading.value = false
  }
}

watch(
  () => route.params.id,
  () => {
    loadGameDetail()
  },
  { immediate: true },
)
</script>

<template>
  <BaseLoading v-if="loading" text="正在加载游戏详情..." />

  <EmptyState
    v-else-if="errorMessage"
    title="游戏不存在或暂时无法加载"
    :description="errorMessage"
  >
    <template #action>
      <RouterLink class="secondary-button" :to="backTarget">返回游戏列表</RouterLink>
    </template>
  </EmptyState>

  <article v-else-if="game" class="game-detail-page">
    <RouterLink class="back-link" :to="backTarget">返回游戏列表</RouterLink>

    <div v-if="mockMessage" class="notice-box is-warning">
      {{ mockMessage }}
    </div>

    <section class="game-detail-hero">
      <div class="detail-cover">
        <img
          v-if="hasCover"
          :src="game.coverUrl"
          :alt="`${game.title} 封面`"
          @error="coverFailed = true"
        />
        <div v-else class="game-cover-placeholder detail-placeholder" aria-hidden="true">
          <span>{{ game.title?.slice(0, 1) || 'N' }}</span>
        </div>
      </div>

      <div class="detail-summary">
        <span v-if="game.category" class="pill">{{ game.category.name }}</span>
        <h1>{{ game.title }}</h1>
        <p>{{ game.description }}</p>

        <dl class="detail-meta">
          <div>
            <dt>开发商</dt>
            <dd>{{ game.developer }}</dd>
          </div>
          <div>
            <dt>发行商</dt>
            <dd>{{ game.publisher }}</dd>
          </div>
          <div>
            <dt>发行日期</dt>
            <dd>{{ game.releaseDate || '待确认' }}</dd>
          </div>
        </dl>

        <div class="tag-list detail-tags" aria-label="游戏标签">
          <span v-for="tag in game.tags" :key="tag.id">{{ tag.name }}</span>
        </div>

        <div class="detail-actions" aria-label="互动操作">
          <LikeButton :game-id="game.id" :initial-count="game.likesCount" @updated="handleLikeUpdated" />
          <FavoriteButton
            :game-id="game.id"
            :initial-count="game.favoritesCount"
            @updated="handleFavoriteUpdated"
          />
        </div>

        <p class="detail-action-tip">
          {{
            isAuthenticated
              ? '当前后端暂未返回“我是否已点赞/已收藏”的读取状态，按钮会在首次操作后同步当前状态。'
              : '登录后可点赞、收藏并参与评论互动。'
          }}
        </p>
      </div>
    </section>

    <ScreenshotCarousel :screenshots="game.screenshots" :title="game.title" />

    <CommentSection :game-id="game.id" />

    <section class="detail-section">
      <div class="section-heading">
        <h2>下载区域</h2>
        <span>Phase 5</span>
      </div>
      <p>下载链接与版本信息将在后续阶段接入真实后端数据。</p>
    </section>
  </article>
</template>
