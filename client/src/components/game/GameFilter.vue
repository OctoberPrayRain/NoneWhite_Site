<script setup>
const props = defineProps({
  categories: {
    type: Array,
    default: () => [],
  },
  tags: {
    type: Array,
    default: () => [],
  },
  categoryId: {
    type: String,
    default: '',
  },
  tagId: {
    type: String,
    default: '',
  },
  disabled: {
    type: Boolean,
    default: false,
  },
})

const emit = defineEmits(['change', 'reset'])

function updateFilter(key, event) {
  emit('change', {
    categoryId: key === 'categoryId' ? event.target.value : props.categoryId,
    tagId: key === 'tagId' ? event.target.value : props.tagId,
  })
}
</script>

<template>
  <section class="game-filter" aria-label="游戏筛选">
    <label class="filter-field">
      <span>分类</span>
      <select :value="categoryId" :disabled="disabled" @change="updateFilter('categoryId', $event)">
        <option value="">全部分类</option>
        <option v-for="category in categories" :key="category.id" :value="String(category.id)">
          {{ category.name }}
        </option>
      </select>
    </label>

    <label class="filter-field">
      <span>标签</span>
      <select :value="tagId" :disabled="disabled" @change="updateFilter('tagId', $event)">
        <option value="">全部标签</option>
        <option v-for="tag in tags" :key="tag.id" :value="String(tag.id)">
          {{ tag.name }}
        </option>
      </select>
    </label>

    <button class="secondary-button" type="button" :disabled="disabled" @click="emit('reset')">
      清空筛选
    </button>
  </section>
</template>
