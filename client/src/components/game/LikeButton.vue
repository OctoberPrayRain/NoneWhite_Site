<script setup>
import { computed, ref, watch } from 'vue'
import { useRoute, useRouter } from 'vue-router'

import { likeGame, unlikeGame } from '../../api/interactions'
import { useAuthStore } from '../../stores/auth'

const props = defineProps({
  gameId: {
    type: [Number, String],
    required: true,
  },
  initialCount: {
    type: Number,
    default: 0,
  },
})

const emit = defineEmits(['updated'])

const route = useRoute()
const router = useRouter()

const { authToken } = useAuthStore()

const liked = ref(false)
const knownState = ref(false)
const count = ref(0)
const loading = ref(false)
const errorMessage = ref('')

const isAuthenticated = computed(() => Boolean(authToken.value))

const buttonLabel = computed(() => {
  if (loading.value) {
    return '处理中...'
  }

  return liked.value ? '已点赞' : '点赞'
})

function resetState() {
  liked.value = false
  knownState.value = false
  count.value = Number(props.initialCount) || 0
  loading.value = false
  errorMessage.value = ''
}

function syncCount(nextCount) {
  count.value = Number(nextCount) || 0
}

function redirectToLogin() {
  router.push({
    path: '/login',
    query: {
      redirect: route.fullPath,
    },
  })
}

async function handleClick() {
  if (!isAuthenticated.value) {
    redirectToLogin()
    return
  }

  loading.value = true
  errorMessage.value = ''

  try {
    const result = knownState.value && liked.value
      ? await unlikeGame(props.gameId, authToken.value)
      : await likeGame(props.gameId, authToken.value)

    liked.value = result.liked
    knownState.value = true
    count.value = result.likesCount
    emit('updated', result)
  } catch (error) {
    errorMessage.value = error instanceof Error ? error.message : '点赞操作失败'
  } finally {
    loading.value = false
  }
}

watch(() => props.gameId, resetState, { immediate: true })
watch(() => props.initialCount, syncCount)
</script>

<template>
  <div class="interaction-stack">
    <button
      class="interaction-button"
      :class="{ 'is-active': liked }"
      type="button"
      :disabled="loading"
      @click="handleClick"
    >
      <span>{{ buttonLabel }}</span>
      <strong>{{ count }}</strong>
    </button>

    <p v-if="errorMessage" class="interaction-feedback is-error">{{ errorMessage }}</p>
  </div>
</template>
