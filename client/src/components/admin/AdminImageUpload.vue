<script setup>
import { ref } from 'vue'

import { uploadAdminImage } from '../../api/admin'
import { useAuthStore } from '../../stores/auth'

const props = defineProps({
  label: {
    type: String,
    default: '上传图片',
  },
  help: {
    type: String,
    default: '支持 PNG、JPEG、WebP，上传成功后会自动填入图片 URL。',
  },
})

const emit = defineEmits(['uploaded'])

const { authToken } = useAuthStore()

const fileInput = ref(null)
const selectedFile = ref(null)
const status = ref('idle')
const message = ref('')
const errorMessage = ref('')

function resetInput() {
  selectedFile.value = null

  if (fileInput.value) {
    fileInput.value.value = ''
  }
}

function handleFileChange(event) {
  const file = event.target.files?.[0]
  message.value = ''
  errorMessage.value = ''

  if (!file) {
    selectedFile.value = null
    status.value = 'idle'
    return
  }

  const allowedTypes = ['image/png', 'image/jpeg', 'image/webp']
  if (!allowedTypes.includes(file.type)) {
    selectedFile.value = null
    status.value = 'error'
    errorMessage.value = '仅支持 PNG、JPEG 或 WebP 图片'
    event.target.value = ''
    return
  }

  selectedFile.value = file
  status.value = 'idle'
}

async function handleUpload() {
  if (!selectedFile.value) {
    status.value = 'error'
    errorMessage.value = '请先选择图片文件'
    return
  }

  status.value = 'loading'
  message.value = ''
  errorMessage.value = ''

  try {
    const result = await uploadAdminImage(selectedFile.value, authToken.value)
    emit('uploaded', result.imageUrl)
    message.value = `已上传：${result.imageUrl}`
    status.value = 'success'
    resetInput()
  } catch (error) {
    status.value = 'error'
    errorMessage.value = error instanceof Error ? error.message : '图片上传失败'
  }
}
</script>

<template>
  <div class="admin-upload-box">
    <label class="form-field">
      <span>{{ label }}</span>
      <input
        ref="fileInput"
        type="file"
        accept="image/png,image/jpeg,image/webp"
        @change="handleFileChange"
      />
    </label>
    <p class="form-help">{{ props.help }}</p>
    <p v-if="status === 'error'" class="notice-box is-error">{{ errorMessage }}</p>
    <p v-if="status === 'success'" class="notice-box is-success">{{ message }}</p>
    <button class="secondary-button" type="button" :disabled="status === 'loading' || !selectedFile" @click="handleUpload">
      {{ status === 'loading' ? '上传中...' : '上传并填入' }}
    </button>
  </div>
</template>
