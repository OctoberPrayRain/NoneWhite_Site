<script setup>
import { computed, onMounted, ref, watch } from 'vue'
import { RouterLink } from 'vue-router'

import { changePassword, updateCurrentUser, uploadAvatar } from '../api/users'
import ProfileFavoritesPanel from '../components/profile/ProfileFavoritesPanel.vue'
import { useAuthStore } from '../stores/auth'

const {
  authToken,
  currentUser,
  authStatus,
  authErrorMessage,
  loadCurrentUser,
  logout,
} = useAuthStore()

const activeTab = ref('profile')
const profileUsername = ref('')
const profileStatus = ref('idle')
const profileMessage = ref('')
const profileErrorMessage = ref('')
const currentPassword = ref('')
const newPassword = ref('')
const confirmNewPassword = ref('')
const passwordStatus = ref('idle')
const passwordMessage = ref('')
const passwordErrorMessage = ref('')

const avatarFileInput = ref(null)
const avatarFile = ref(null)
const avatarStatus = ref('idle')
const avatarMessage = ref('')
const avatarErrorMessage = ref('')

const isAuthenticated = computed(() => Boolean(authToken.value))
const displayInitial = computed(() => currentUser.value?.username?.slice(0, 1).toUpperCase() || 'N')

function syncProfileForm() {
  profileUsername.value = currentUser.value?.username || ''
}

function formatDate(value) {
  if (!value) {
    return '暂未返回'
  }

  return new Date(value).toLocaleString('zh-CN')
}

function handleAvatarChange(event) {
  const file = event.target.files?.[0]

  avatarMessage.value = ''

  if (!file) {
    avatarFile.value = null
    avatarErrorMessage.value = ''
    avatarStatus.value = 'idle'
    return
  }

  const allowedTypes = ['image/png', 'image/jpeg', 'image/webp']
  if (!allowedTypes.includes(file.type)) {
    avatarFile.value = null
    avatarErrorMessage.value = '仅支持 PNG、JPEG 或 WebP 头像文件'
    avatarStatus.value = 'error'
    event.target.value = ''
    return
  }

  if (file.size > 2097152) {
    avatarFile.value = null
    avatarErrorMessage.value = '头像文件不能超过 2 MiB'
    avatarStatus.value = 'error'
    event.target.value = ''
    return
  }

  avatarFile.value = file
  avatarErrorMessage.value = ''
  avatarStatus.value = 'idle'
}

async function handleAvatarUpload() {
  if (!avatarFile.value) {
    avatarErrorMessage.value = '请先选择头像文件'
    avatarStatus.value = 'error'
    return
  }

  avatarStatus.value = 'loading'
  avatarMessage.value = ''
  avatarErrorMessage.value = ''

  try {
    const result = await uploadAvatar(avatarFile.value, authToken.value)

    currentUser.value = {
      ...(currentUser.value || {}),
      avatarUrl: result.avatarUrl,
    }
    avatarMessage.value = '头像已上传，预览已更新'
    avatarStatus.value = 'success'
    avatarFile.value = null

    if (avatarFileInput.value) {
      avatarFileInput.value.value = ''
    }
  } catch (error) {
    avatarErrorMessage.value = error instanceof Error ? error.message : '头像上传失败'
    avatarStatus.value = 'error'
  }
}

async function refreshProfile() {
  if (!authToken.value) {
    return
  }

  profileStatus.value = 'loading'
  profileErrorMessage.value = ''

  try {
    await loadCurrentUser()
    syncProfileForm()
    profileStatus.value = 'success'
  } catch (error) {
    profileErrorMessage.value = error instanceof Error ? error.message : '加载个人资料失败'
    profileStatus.value = 'error'
  }
}

async function handleUpdateProfile() {
  const username = profileUsername.value.trim()

  if (username.length < 3) {
    profileErrorMessage.value = '用户名至少需要 3 个字符'
    profileStatus.value = 'error'
    return
  }

  profileStatus.value = 'loading'
  profileMessage.value = ''
  profileErrorMessage.value = ''

  try {
    currentUser.value = await updateCurrentUser({ username }, authToken.value)
    syncProfileForm()
    profileMessage.value = '个人资料已更新'
    profileStatus.value = 'success'
  } catch (error) {
    profileErrorMessage.value = error instanceof Error ? error.message : '更新个人资料失败'
    profileStatus.value = 'error'
  }
}

async function handleChangePassword() {
  if (!currentPassword.value) {
    passwordErrorMessage.value = '请输入当前密码'
    passwordStatus.value = 'error'
    return
  }

  if (newPassword.value.length < 8) {
    passwordErrorMessage.value = '新密码至少需要 8 个字符'
    passwordStatus.value = 'error'
    return
  }

  if (newPassword.value === currentPassword.value) {
    passwordErrorMessage.value = '新密码不能和当前密码相同'
    passwordStatus.value = 'error'
    return
  }

  if (newPassword.value !== confirmNewPassword.value) {
    passwordErrorMessage.value = '两次输入的新密码不一致'
    passwordStatus.value = 'error'
    return
  }

  passwordStatus.value = 'loading'
  passwordMessage.value = ''
  passwordErrorMessage.value = ''

  try {
    await changePassword(
      {
        currentPassword: currentPassword.value,
        newPassword: newPassword.value,
      },
      authToken.value,
    )
    currentPassword.value = ''
    newPassword.value = ''
    confirmNewPassword.value = ''
    passwordMessage.value = '密码已修改，请妥善保存新密码'
    passwordStatus.value = 'success'
  } catch (error) {
    passwordErrorMessage.value = error instanceof Error ? error.message : '修改密码失败'
    passwordStatus.value = 'error'
  }
}

watch(currentUser, syncProfileForm)
onMounted(refreshProfile)
</script>

<template>
  <section class="page-heading profile-heading">
    <div class="eyebrow">User Center</div>
    <h1>个人中心</h1>
    <p>这里接入用户系统与 Phase 4 收藏列表能力，可继续维护资料、密码与头像，并查看已收藏的游戏。</p>
  </section>

  <section v-if="!isAuthenticated" class="status-panel auth-empty-panel">
    <div class="status-header">
      <span class="status-dot" aria-hidden="true"></span>
      <h2>尚未登录</h2>
    </div>
    <p class="status-message">登录后可以查看个人资料、修改用户名和修改密码。</p>
    <div class="hero-actions">
      <RouterLink class="primary-button" to="/login">去登录</RouterLink>
      <RouterLink class="secondary-button" to="/register">注册账号</RouterLink>
    </div>
  </section>

  <section v-else class="profile-layout">
    <aside class="profile-card account-card">
      <div class="avatar-preview">
        <img v-if="currentUser?.avatarUrl" :src="currentUser.avatarUrl" alt="用户头像" />
        <span v-else>{{ displayInitial }}</span>
      </div>
      <h2>{{ currentUser?.username || '正在加载用户' }}</h2>
      <p>{{ currentUser?.email || '邮箱暂未加载' }}</p>
      <dl class="profile-meta">
        <div>
          <dt>ID</dt>
          <dd>{{ currentUser?.id || '暂未返回' }}</dd>
        </div>
        <div>
          <dt>角色</dt>
          <dd>{{ currentUser?.role || 'user' }}</dd>
        </div>
        <div>
          <dt>创建时间</dt>
          <dd>{{ formatDate(currentUser?.createdAt) }}</dd>
        </div>
        <div>
          <dt>更新时间</dt>
          <dd>{{ formatDate(currentUser?.updatedAt) }}</dd>
        </div>
      </dl>
      <button class="secondary-button" type="button" @click="refreshProfile">
        {{ authStatus === 'loading' ? '刷新中...' : '刷新资料' }}
      </button>
      <button class="ghost-button" type="button" @click="logout">退出登录</button>
      <p v-if="authStatus === 'error'" class="notice-box is-error">{{ authErrorMessage }}</p>
    </aside>

    <div class="profile-main">
      <div class="profile-tabs" role="tablist" aria-label="个人中心选项卡">
        <button :class="{ active: activeTab === 'profile' }" type="button" @click="activeTab = 'profile'">
          资料设置
        </button>
        <button :class="{ active: activeTab === 'favorites' }" type="button" @click="activeTab = 'favorites'">
          收藏列表
        </button>
      </div>

      <div v-if="activeTab === 'profile'" class="profile-grid">
        <form class="profile-card" @submit.prevent="handleUpdateProfile">
          <div class="form-heading">
            <span class="pill">Profile</span>
            <h2>编辑资料</h2>
          </div>
          <label class="form-field">
            <span>用户名</span>
            <input v-model="profileUsername" type="text" autocomplete="username" />
          </label>
          <p class="form-help">Phase 2 仅允许修改用户名，不提交邮箱或头像 URL。</p>
          <p v-if="profileStatus === 'error'" class="notice-box is-error">{{ profileErrorMessage }}</p>
          <p v-if="profileStatus === 'success' && profileMessage" class="notice-box is-success">
            {{ profileMessage }}
          </p>
          <button class="primary-button form-button" type="submit" :disabled="profileStatus === 'loading'">
            {{ profileStatus === 'loading' ? '保存中...' : '保存资料' }}
          </button>
        </form>

        <form class="profile-card" @submit.prevent="handleChangePassword">
          <div class="form-heading">
            <span class="pill">Password</span>
            <h2>修改密码</h2>
          </div>
          <label class="form-field">
            <span>当前密码</span>
            <input v-model="currentPassword" type="password" autocomplete="current-password" />
          </label>
          <label class="form-field">
            <span>新密码</span>
            <input v-model="newPassword" type="password" autocomplete="new-password" />
          </label>
          <label class="form-field">
            <span>确认新密码</span>
            <input v-model="confirmNewPassword" type="password" autocomplete="new-password" />
          </label>
          <p v-if="passwordStatus === 'error'" class="notice-box is-error">{{ passwordErrorMessage }}</p>
          <p v-if="passwordStatus === 'success'" class="notice-box is-success">{{ passwordMessage }}</p>
          <button class="primary-button form-button" type="submit" :disabled="passwordStatus === 'loading'">
            {{ passwordStatus === 'loading' ? '修改中...' : '修改密码' }}
          </button>
        </form>

        <form class="profile-card" @submit.prevent="handleAvatarUpload">
          <div class="form-heading">
            <span class="pill">Avatar</span>
            <h2>头像上传</h2>
          </div>
          <label class="form-field">
            <span>选择头像文件（PNG / JPEG / WebP，最大 2 MiB）</span>
            <input
              ref="avatarFileInput"
              type="file"
              accept="image/png,image/jpeg,image/webp"
              @change="handleAvatarChange"
            />
          </label>
          <p class="form-help">上传会使用 <code>avatar</code> 字段提交到后端，并用返回的 avatarUrl 更新上方头像预览。</p>
          <p v-if="avatarStatus === 'error'" class="notice-box is-error">{{ avatarErrorMessage }}</p>
          <p v-if="avatarStatus === 'success' && avatarMessage" class="notice-box is-success">
            {{ avatarMessage }}
          </p>
          <button class="primary-button form-button" type="submit" :disabled="avatarStatus === 'loading' || !avatarFile">
            {{ avatarStatus === 'loading' ? '上传中...' : '上传头像' }}
          </button>
        </form>
      </div>

      <ProfileFavoritesPanel v-else :active="activeTab === 'favorites'" />
    </div>
  </section>
</template>
