<script setup>
import { computed } from 'vue'

defineOptions({
  name: 'CommentThreadItem',
})

const props = defineProps({
  comment: {
    type: Object,
    required: true,
  },
  currentUserId: {
    type: Number,
    default: 0,
  },
  deletingId: {
    type: Number,
    default: 0,
  },
  replyTargetId: {
    type: Number,
    default: 0,
  },
  level: {
    type: Number,
    default: 0,
  },
})

const emit = defineEmits(['reply', 'delete'])

const isOwner = computed(
  () => Boolean(props.currentUserId) && Number(props.comment.userId) === Number(props.currentUserId),
)
const hasAvatar = computed(() => Boolean(props.comment.avatarUrl))
const displayInitial = computed(() => props.comment.username?.slice(0, 1).toUpperCase() || 'N')
const nestingLevel = computed(() => Math.min(props.level, 4))
const createdAtText = computed(() => formatDate(props.comment.createdAt))

function formatDate(value) {
  if (!value) {
    return '刚刚'
  }

  return new Date(value).toLocaleString('zh-CN')
}

function handleReply() {
  emit('reply', props.comment)
}

function handleDelete() {
  emit('delete', props.comment)
}
</script>

<template>
  <article
    class="comment-card"
    :class="{
      'is-nested': level > 0,
      'is-reply-target': replyTargetId === comment.id,
    }"
    :style="{ '--comment-level': nestingLevel }"
  >
    <div class="comment-card__header">
      <div class="comment-author">
        <div class="comment-avatar">
          <img v-if="hasAvatar" :src="comment.avatarUrl" :alt="`${comment.username} 的头像`" />
          <span v-else>{{ displayInitial }}</span>
        </div>

        <div class="comment-author__meta">
          <strong>{{ comment.username }}</strong>
          <div class="comment-author__subline">
            <span>{{ createdAtText }}</span>
            <span v-if="comment.parentId" class="comment-badge">回复评论</span>
          </div>
        </div>
      </div>

      <div class="comment-toolbar">
        <button class="ghost-button comment-action" type="button" @click="handleReply">回复</button>
        <button
          v-if="isOwner"
          class="ghost-button comment-action danger"
          type="button"
          :disabled="deletingId === comment.id"
          @click="handleDelete"
        >
          {{ deletingId === comment.id ? '删除中...' : '删除' }}
        </button>
      </div>
    </div>

    <p class="comment-content">{{ comment.content }}</p>

    <div v-if="comment.children?.length" class="comment-children">
      <CommentThreadItem
        v-for="child in comment.children"
        :key="child.id"
        :comment="child"
        :current-user-id="currentUserId"
        :deleting-id="deletingId"
        :reply-target-id="replyTargetId"
        :level="level + 1"
        @reply="$emit('reply', $event)"
        @delete="$emit('delete', $event)"
      />
    </div>
  </article>
</template>
