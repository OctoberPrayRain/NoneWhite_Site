<template>
  <div class="empty-state">
    <p class="empty-title">{{ title }}</p>
    <p v-if="description" class="empty-description">{{ description }}</p>
    <div v-if="$slots.action" class="empty-action">
      <slot name="action"></slot>
    </div>
  </div>
</template>

<script setup>
defineProps({
  title: {
    type: String,
    default: '暂无内容',
  },
  description: {
    type: String,
    default: '',
  },
})
</script>
