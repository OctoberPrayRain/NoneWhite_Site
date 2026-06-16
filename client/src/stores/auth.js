import { ref } from 'vue'

import { login, logoutLocal, register } from '../api/auth'
import { ApiError } from '../api/http'
import { fetchCurrentUser } from '../api/users'

export const AUTH_TOKEN_STORAGE_KEY = 'nonewhite_auth_token'

const authToken = ref('')
const tokenType = ref('Bearer')
const expiresIn = ref(null)
const currentUser = ref(null)
const authStatus = ref('idle')
const authErrorMessage = ref('')
let tokenLoaded = false

function readStoredToken() {
  if (typeof window === 'undefined') {
    return ''
  }

  return window.localStorage.getItem(AUTH_TOKEN_STORAGE_KEY) || ''
}

function writeStoredToken(token) {
  if (typeof window !== 'undefined') {
    window.localStorage.setItem(AUTH_TOKEN_STORAGE_KEY, token)
  }
}

function removeStoredToken() {
  if (typeof window !== 'undefined') {
    window.localStorage.removeItem(AUTH_TOKEN_STORAGE_KEY)
  }
}

function toErrorMessage(error) {
  return error instanceof Error ? error.message : '请求失败，请稍后重试'
}

export function loadTokenFromStorage() {
  authToken.value = readStoredToken()
  tokenLoaded = true
  return authToken.value
}

export function saveToken(token) {
  authToken.value = token
  writeStoredToken(token)
}

export function clearAuth() {
  authToken.value = ''
  tokenType.value = 'Bearer'
  expiresIn.value = null
  currentUser.value = null
  authStatus.value = 'idle'
  authErrorMessage.value = ''
  removeStoredToken()
  logoutLocal()
}

export async function loginWithCredentials(payload) {
  authStatus.value = 'loading'
  authErrorMessage.value = ''

  try {
    const data = await login(payload)
    saveToken(data.token)
    tokenType.value = data.tokenType || 'Bearer'
    expiresIn.value = data.expiresIn || null
    currentUser.value = data.user
    authStatus.value = 'success'
    return data
  } catch (error) {
    authErrorMessage.value = toErrorMessage(error)
    authStatus.value = 'error'
    throw error
  }
}

export async function registerWithCredentials(payload) {
  authStatus.value = 'loading'
  authErrorMessage.value = ''

  try {
    await register(payload)
    const data = await login({
      email: payload.email,
      password: payload.password,
    })
    saveToken(data.token)
    tokenType.value = data.tokenType || 'Bearer'
    expiresIn.value = data.expiresIn || null
    currentUser.value = data.user
    authStatus.value = 'success'
    return data
  } catch (error) {
    authErrorMessage.value = toErrorMessage(error)
    authStatus.value = 'error'
    throw error
  }
}

export async function loadCurrentUser() {
  if (!authToken.value) {
    currentUser.value = null
    authStatus.value = 'empty'
    return null
  }

  authStatus.value = 'loading'
  authErrorMessage.value = ''

  try {
    currentUser.value = await fetchCurrentUser(authToken.value)
    authStatus.value = 'success'
    return currentUser.value
  } catch (error) {
    const errorMessage = toErrorMessage(error)

    if (error instanceof ApiError && error.status === 401) {
      clearAuth()
    }

    authErrorMessage.value = errorMessage
    authStatus.value = 'error'
    throw error
  }
}

export function logout() {
  clearAuth()
}

export function useAuthStore() {
  if (!tokenLoaded) {
    loadTokenFromStorage()
  }

  return {
    authToken,
    tokenType,
    expiresIn,
    currentUser,
    authStatus,
    authErrorMessage,
    loadTokenFromStorage,
    saveToken,
    clearAuth,
    loginWithCredentials,
    registerWithCredentials,
    loadCurrentUser,
    logout,
  }
}
