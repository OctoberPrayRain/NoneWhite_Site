<script setup>
import { computed, ref } from 'vue'

const props = defineProps({
  game: {
    type: Object,
    required: true,
  },
  query: {
    type: Object,
    default: () => ({}),
  },
})

const imageFailed = ref(false)

const detailTarget = computed(() => ({
  name: 'game-detail',
  params: {
    id: props.game.id,
  },
  query: props.query,
}))

const hasCover = computed(() => props.game.coverUrl && !imageFailed.value)
</script>

<template>
  <RouterLink class="game-card" :to="detailTarget">
    <div class="game-card-cover">
      <img
        v-if="hasCover"
        :src="game.coverUrl"
        :alt="`${game.title} 封面`"
        loading="lazy"
        @error="imageFailed = true"
      />
      <div v-else class="game-cover-placeholder" aria-hidden="true">
        <span>{{ game.title?.slice(0, 1) || 'N' }}</span>
      </div>
    </div>

    <div class="game-card-body">
      <div class="game-card-heading">
        <span v-if="game.category" class="pill mini-pill">{{ game.category.name }}</span>
        <h2>{{ game.title }}</h2>
      </div>
      <p>{{ game.description }}</p>
      <div class="tag-list" aria-label="文件标签">
        <span v-for="tag in game.tags" :key="tag.id">{{ tag.name }}</span>
      </div>
      <dl class="game-card-meta">
        <div>
          <dt>提供方</dt>
          <dd>{{ game.developer }}</dd>
        </div>
        <div>
          <dt>发布日期</dt>
          <dd>{{ game.releaseDate || '待确认' }}</dd>
        </div>
      </dl>
      <div class="game-stat-row" aria-label="互动数据">
        <span>点赞 {{ game.likesCount }}</span>
        <span>收藏 {{ game.favoritesCount }}</span>
      </div>
    </div>
  </RouterLink>
</template>
