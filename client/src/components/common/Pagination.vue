<script setup>
import { computed } from 'vue'

const props = defineProps({
  page: {
    type: Number,
    required: true,
  },
  pageSize: {
    type: Number,
    required: true,
  },
  total: {
    type: Number,
    required: true,
  },
  disabled: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['update:page'])

const totalPages = computed(() => Math.max(1, Math.ceil(props.total / props.pageSize)))

const pages = computed(() => {
  const start = Math.max(1, props.page - 2)
  const end = Math.min(totalPages.value, start + 4)
  return Array.from({ length: end - start + 1 }, (_, index) => start + index)
})

function changePage(nextPage) {
  if (props.disabled || nextPage === props.page || nextPage < 1 || nextPage > totalPages.value) {
    return
  }

  emit('update:page', nextPage)
}
</script>

<template>
  <nav class="pagination" aria-label="分页导航">
    <button type="button" :disabled="disabled || page <= 1" @click="changePage(page - 1)">
      上一页
    </button>
    <button
      v-for="item in pages"
      :key="item"
      type="button"
      :class="{ active: item === page }"
      :disabled="disabled || item === page"
      @click="changePage(item)"
    >
      {{ item }}
    </button>
    <button type="button" :disabled="disabled || page >= totalPages" @click="changePage(page + 1)">
      下一页
    </button>
  </nav>
</template>
