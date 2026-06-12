<script setup>
import { computed, nextTick, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { createComment, deleteComment, getComments } from '../../api/interactions'
import { useAuthStore } from '../../stores/auth'
import BaseLoading from '../common/BaseLoading.vue'
import EmptyState from '../common/EmptyState.vue'
import Pagination from '../common/Pagination.vue'
import CommentThreadItem from './CommentThreadItem.vue'

const PAGE_SIZE = 12

const props = defineProps({
  gameId: {
    type: [Number, String],
    required: true,
  },
})

const route = useRoute()
const router = useRouter()

const { authToken, currentUser, loadCurrentUser } = useAuthStore()

const comments = ref([])
const total = ref(0)
const page = ref(1)
const loading = ref(false)
const loadedOnce = ref(false)
const errorMessage = ref('')
const submitStatus = ref('idle')
const submitMessage = ref('')
const submitErrorMessage = ref('')
const replyTargetId = ref(null)
const content = ref('')
const deletingId = ref(0)
const textareaRef = ref(null)

const isAuthenticated = computed(() => Boolean(authToken.value))
const currentUserId = computed(() => Number(currentUser.value?.id ?? 0))
const hasComments = computed(() => comments.value.length > 0)
const replyTargetComment = computed(() => comments.value.find((item) => item.id === replyTargetId.value) || null)
const commentTree = computed(() => buildCommentTree(comments.value))

function toErrorMessage(error) {
  return error instanceof Error ? error.message : '请求失败，请稍后重试'
}

function buildCommentTree(list) {
  const commentMap = new Map(list.map((item) => [item.id, { ...item, children: [] }]))
  const roots = []

  for (const item of list) {
    const node = commentMap.get(item.id)

    if (item.parentId && commentMap.has(item.parentId)) {
      commentMap.get(item.parentId).children.push(node)
      continue
    }

    roots.push(node)
  }

  return roots
}

function buildLoginTarget() {
  return {
    path: '/login',
    query: {
      redirect: route.fullPath,
    },
  }
}

async function ensureCurrentUserLoaded() {
  if (!authToken.value || currentUser.value) {
    return
  }

  try {
    await loadCurrentUser()
  } catch {
    // Auth store already handles invalid-token cleanup.
  }
}

async function loadComments() {
  loading.value = true
  errorMessage.value = ''

  try {
    const result = await getComments(props.gameId, {
      page: page.value,
      pageSize: PAGE_SIZE,
    })

    comments.value = result.list
    total.value = result.total
    page.value = result.page
    loadedOnce.value = true
  } catch (error) {
    comments.value = []
    total.value = 0
    errorMessage.value = toErrorMessage(error)
  } finally {
    loading.value = false
  }
}

function focusComposer() {
  nextTick(() => {
    textareaRef.value?.focus?.()
  })
}

function clearReplyTarget() {
  replyTargetId.value = null
}

function handleReply(comment) {
  if (!isAuthenticated.value) {
    router.push(buildLoginTarget())
    return
  }

  replyTargetId.value = comment.id
  submitMessage.value = ''
  submitErrorMessage.value = ''
  submitStatus.value = 'idle'
  focusComposer()
}

async function handleSubmit() {
  if (!isAuthenticated.value) {
    router.push(buildLoginTarget())
    return
  }

  const trimmedContent = content.value.trim()

  if (!trimmedContent) {
    submitStatus.value = 'error'
    submitErrorMessage.value = '请输入评论内容'
    return
  }

  submitStatus.value = 'loading'
  submitMessage.value = ''
  submitErrorMessage.value = ''

  try {
    const result = await createComment(
      props.gameId,
      {
        content: trimmedContent,
        parentId: replyTargetId.value,
      },
      authToken.value,
    )

    content.value = ''
    replyTargetId.value = null
    page.value = 1
    await loadComments()
    submitStatus.value = 'success'
    submitMessage.value = result.parentId ? '回复已发送' : '评论已发布'
  } catch (error) {
    submitStatus.value = 'error'
    submitErrorMessage.value = toErrorMessage(error)
  }
}

async function handleDelete(comment) {
  deletingId.value = comment.id
  errorMessage.value = ''

  try {
    await deleteComment(comment.id, authToken.value)
    const nextTotal = Math.max(0, total.value - 1)
    const lastPage = Math.max(1, Math.ceil(nextTotal / PAGE_SIZE))

    if (page.value > lastPage) {
      page.value = lastPage
    }

    await loadComments()
    submitStatus.value = 'success'
    submitMessage.value = '评论已删除'
    submitErrorMessage.value = ''
  } catch (error) {
    errorMessage.value = toErrorMessage(error)
  } finally {
    deletingId.value = 0
  }
}

function handlePageChange(nextPage) {
  page.value = nextPage
  loadComments()
}

watch(() => authToken.value, ensureCurrentUserLoaded, { immediate: true })
watch(
  () => props.gameId,
  () => {
    comments.value = []
    total.value = 0
    page.value = 1
    replyTargetId.value = null
    content.value = ''
    submitStatus.value = 'idle'
    submitMessage.value = ''
    submitErrorMessage.value = ''
    loadComments()
  },
  { immediate: true },
)
</script>

<template>
  <section class="detail-section comment-section">
    <div class="section-heading section-heading--stacked">
      <div>
        <h2>评论区</h2>
        <p>支持发表、回复和删除自己的评论，评论列表按最新时间展示。</p>
      </div>
      <span>{{ total }} 条评论</span>
    </div>

    <div class="comment-composer">
      <template v-if="isAuthenticated">
        <div v-if="replyTargetComment" class="comment-reply-banner">
          <p>
            正在回复 <strong>{{ replyTargetComment.username }}</strong>
          </p>
          <button class="secondary-button" type="button" @click="clearReplyTarget">取消回复</button>
        </div>

        <label class="form-field">
          <span>{{ replyTargetComment ? '回复内容' : '发表评论' }}</span>
          <textarea
            ref="textareaRef"
            v-model="content"
            rows="5"
            maxlength="1000"
            placeholder="写下你对这部作品的看法..."
          ></textarea>
        </label>

        <div class="comment-composer__footer">
          <p class="form-help">评论会在提交后立即刷新，单条内容最多 1000 个字符。</p>
          <span class="comment-counter">{{ content.trim().length }}/1000</span>
        </div>

        <p v-if="submitStatus === 'error'" class="notice-box is-error">{{ submitErrorMessage }}</p>
        <p v-if="submitStatus === 'success' && submitMessage" class="notice-box is-success">
          {{ submitMessage }}
        </p>

        <div class="comment-composer__actions">
          <button v-if="replyTargetComment" class="secondary-button" type="button" @click="clearReplyTarget">
            取消回复
          </button>
          <button class="primary-button" type="button" :disabled="submitStatus === 'loading'" @click="handleSubmit">
            {{ submitStatus === 'loading' ? '提交中...' : replyTargetComment ? '发送回复' : '发布评论' }}
          </button>
        </div>
      </template>

      <template v-else>
        <div class="comment-login-box">
          <p>登录后可以发表评论、回复他人，并删除自己发布的评论。</p>
          <div class="hero-actions">
            <RouterLink class="primary-button" :to="buildLoginTarget()">去登录</RouterLink>
            <RouterLink class="secondary-button" to="/register">注册账号</RouterLink>
          </div>
        </div>
      </template>
    </div>

    <div v-if="errorMessage && hasComments" class="notice-box is-error">
      {{ errorMessage }}
    </div>

    <BaseLoading v-if="loading && !hasComments" text="正在加载评论..." />

    <div v-else-if="errorMessage && !hasComments" class="status-panel is-error compact-panel">
      <div class="status-header">
        <span class="status-dot" aria-hidden="true"></span>
        <h2>评论加载失败</h2>
      </div>
      <p class="status-message">{{ errorMessage }}</p>
      <button class="secondary-button" type="button" @click="loadComments">重新加载</button>
    </div>

    <EmptyState
      v-else-if="loadedOnce && !hasComments"
      title="还没有评论"
      description="成为第一位留言的人，分享你对这部作品的体验。"
    />

    <template v-else>
      <div class="comment-list">
        <CommentThreadItem
          v-for="comment in commentTree"
          :key="comment.id"
          :comment="comment"
          :current-user-id="currentUserId"
          :deleting-id="deletingId"
          :reply-target-id="replyTargetId || 0"
          @reply="handleReply"
          @delete="handleDelete"
        />
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
