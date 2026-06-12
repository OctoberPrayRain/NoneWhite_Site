<template>
  <div class="loading-panel" role="status" aria-live="polite">
    <span class="loading-dot" aria-hidden="true"></span>
    <span>{{ text }}</span>
  </div>
</template>

<script setup>
defineProps({
  text: {
    type: String,
    default: '加载中...',
  },
})
</script>
