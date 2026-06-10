<script setup>
import { computed, ref, watch } from 'vue'

const props = defineProps({
  screenshots: {
    type: Array,
    default: () => [],
  },
  title: {
    type: String,
    default: '游戏截图',
  },
})

const activeIndex = ref(0)
const imageFailedMap = ref({})

const orderedScreenshots = computed(() =>
  [...props.screenshots].sort((a, b) => (a.sortOrder ?? 0) - (b.sortOrder ?? 0)),
)

const activeScreenshot = computed(() => orderedScreenshots.value[activeIndex.value])

watch(
  () => props.screenshots,
  () => {
    activeIndex.value = 0
    imageFailedMap.value = {}
  },
)

function showPrevious() {
  activeIndex.value =
    activeIndex.value === 0 ? orderedScreenshots.value.length - 1 : activeIndex.value - 1
}

function showNext() {
  activeIndex.value =
    activeIndex.value === orderedScreenshots.value.length - 1 ? 0 : activeIndex.value + 1
}

function markFailed(id) {
  imageFailedMap.value = {
    ...imageFailedMap.value,
    [id]: true,
  }
}
</script>

<template>
  <section class="screenshot-section">
    <div class="section-heading">
      <h2>截图预览</h2>
      <span v-if="orderedScreenshots.length">{{ activeIndex + 1 }} / {{ orderedScreenshots.length }}</span>
    </div>

    <div v-if="!orderedScreenshots.length" class="screenshot-empty">暂无截图</div>
    <div v-else class="screenshot-carousel">
      <button
        class="carousel-button"
        type="button"
        :disabled="orderedScreenshots.length <= 1"
        aria-label="上一张截图"
        @click="showPrevious"
      >
        ‹
      </button>

      <div class="screenshot-frame">
        <img
          v-if="activeScreenshot?.url && !imageFailedMap[activeScreenshot.id]"
          :src="activeScreenshot.url"
          :alt="`${title} 截图 ${activeIndex + 1}`"
          @error="markFailed(activeScreenshot.id)"
        />
        <div v-else class="screenshot-placeholder">
          <span>{{ title }}</span>
        </div>
      </div>

      <button
        class="carousel-button"
        type="button"
        :disabled="orderedScreenshots.length <= 1"
        aria-label="下一张截图"
        @click="showNext"
      >
        ›
      </button>
    </div>
  </section>
</template>
