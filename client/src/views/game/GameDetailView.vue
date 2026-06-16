<script setup>
import { computed, ref, watch } from 'vue'
import { useRoute } from 'vue-router'

import { getGameDetail, getPublicDownloadLinks } from '../../api/games'
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
const downloadLinks = ref([])
const downloadLoading = ref(false)
const downloadErrorMessage = ref('')

const backTarget = computed(() => ({
  name: 'games',
  query: route.query,
}))

const hasCover = computed(() => game.value?.coverUrl && !coverFailed.value)
const hasDownloadLinks = computed(() => downloadLinks.value.length > 0)
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

let gameDetailRequestId = 0
let downloadLinksRequestId = 0

function isCurrentGameDetailRequest(requestId, routeId) {
  return requestId === gameDetailRequestId && route.params.id === routeId
}

function isCurrentDownloadLinksRequest(requestId, gameId) {
  return requestId === downloadLinksRequestId && game.value?.id === gameId
}

async function loadDownloadLinks(gameId) {
  const currentReqId = ++downloadLinksRequestId

  downloadLoading.value = true
  downloadErrorMessage.value = ''
  downloadLinks.value = []

  try {
    const links = await getPublicDownloadLinks(gameId)
    if (isCurrentDownloadLinksRequest(currentReqId, gameId)) {
      downloadLinks.value = links
    }
  } catch (error) {
    if (isCurrentDownloadLinksRequest(currentReqId, gameId)) {
      downloadErrorMessage.value = error instanceof Error ? error.message : '下载链接加载失败'
    }
  } finally {
    if (isCurrentDownloadLinksRequest(currentReqId, gameId)) {
      downloadLoading.value = false
    }
  }
}

async function loadGameDetail() {
  const currentReqId = ++gameDetailRequestId
  const currentRouteId = route.params.id
  downloadLinksRequestId += 1

  loading.value = true
  errorMessage.value = ''
  mockMessage.value = ''
  coverFailed.value = false
  game.value = null
  downloadLinks.value = []
  downloadErrorMessage.value = ''
  downloadLoading.value = false

  try {
    const result = await getGameDetail(currentRouteId)
    if (isCurrentGameDetailRequest(currentReqId, currentRouteId)) {
      game.value = result.game

      if (result.isMock) {
        mockMessage.value = '由于网络或服务原因，当前为您展示离线预览详情。'
      }

      loadDownloadLinks(result.game.id)
    }
  } catch (error) {
    if (isCurrentGameDetailRequest(currentReqId, currentRouteId)) {
      errorMessage.value = error instanceof Error ? error.message : '游戏详情加载失败'
    }
  } finally {
    if (isCurrentGameDetailRequest(currentReqId, currentRouteId)) {
      loading.value = false
    }
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
              ? '点赞与收藏会在操作后同步到你的账号。'
              : '登录后可点赞、收藏并参与评论互动。'
          }}
        </p>
      </div>
    </section>

    <ScreenshotCarousel :screenshots="game.screenshots" :title="game.title" />

    <CommentSection :game-id="game.id" />

    <section class="detail-section download-section">
      <div class="section-heading">
        <h2>下载区域</h2>
      </div>

      <p v-if="downloadLoading" class="muted-text">正在加载下载链接...</p>
      <div v-else-if="downloadErrorMessage" class="notice-box is-error">
        {{ downloadErrorMessage }}
      </div>
      <EmptyState
        v-else-if="!hasDownloadLinks"
        title="暂无下载链接"
        description="管理员添加网盘信息后，这里会展示平台、提取码和文件大小。"
      />
      <div v-else class="download-link-grid">
        <article v-for="link in downloadLinks" :key="link.id" class="download-link-card">
          <div>
            <strong>{{ link.platform }}</strong>
            <span>{{ link.fileSize || '未标注大小' }}</span>
          </div>
          <a :href="link.url" target="_blank" rel="noreferrer">下载资源</a>
          <dl>
            <div>
              <dt>提取码</dt>
              <dd>{{ link.extractCode || '无' }}</dd>
            </div>
            <div>
              <dt>密码</dt>
              <dd>{{ link.password || '无' }}</dd>
            </div>
          </dl>
        </article>
      </div>
    </section>
  </article>
</template>
