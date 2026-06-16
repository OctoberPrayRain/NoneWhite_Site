#!/usr/bin/env node

import assert from 'node:assert/strict'

import { createServer } from 'vite'

function createLocalStorage() {
  const values = new Map()

  return {
    getItem(key) {
      return values.has(key) ? values.get(key) : null
    },
    setItem(key, value) {
      values.set(key, String(value))
    },
    removeItem(key) {
      values.delete(key)
    },
    clear() {
      values.clear()
    },
  }
}

function jsonResponse(status, body) {
  return {
    ok: status >= 200 && status < 300,
    status,
    async text() {
      return JSON.stringify(body)
    },
  }
}

async function runScenario(name, scenario) {
  try {
    await scenario()
    console.log(`PASS ${name}`)
  } catch (error) {
    console.error(`FAIL ${name}`)
    throw error
  }
}

globalThis.window = { localStorage: createLocalStorage() }

const vite = await createServer({
  configFile: false,
  root: new URL('..', import.meta.url).pathname,
  server: { middlewareMode: true },
})

const authStore = await vite.ssrLoadModule('/src/stores/auth.js')
const store = authStore.useAuthStore()
const {
  authToken,
  currentUser,
  registerWithCredentials,
  loginWithCredentials,
  clearAuth,
} = store
const { AUTH_TOKEN_STORAGE_KEY } = authStore

const user = {
  id: 17,
  username: 'auto-user',
  email: 'auto@example.com',
  avatarUrl: null,
  role: 'user',
  createdAt: '2026-06-16T00:00:00Z',
  updatedAt: '2026-06-16T00:00:00Z',
}

await runScenario('register_auto_login_success', async () => {
  clearAuth()
  const calls = []
  globalThis.fetch = async (path, options) => {
    calls.push({ path, body: options?.body })
    if (path === '/api/auth/register') {
      return jsonResponse(201, { code: 0, data: user, message: 'User registered successfully' })
    }
    if (path === '/api/auth/login') {
      return jsonResponse(200, {
        code: 0,
        data: { token: 'token-after-register', tokenType: 'Bearer', expiresIn: 3600, user },
        message: 'Login successful',
      })
    }
    throw new Error(`Unexpected fetch path: ${path}`)
  }

  const result = await registerWithCredentials({
    username: user.username,
    email: user.email,
    password: 'password123',
  })

  assert.equal(authToken.value, 'token-after-register', 'registerWithCredentials should persist login token after successful registration')
  assert.equal(window.localStorage.getItem(AUTH_TOKEN_STORAGE_KEY), 'token-after-register', 'registerWithCredentials should persist token to localStorage')
  assert.equal(currentUser.value?.email, user.email, 'registerWithCredentials should set currentUser from login response')
  assert.equal(result.token, 'token-after-register', 'registerWithCredentials should return the login token response')
  assert.deepEqual(calls.map((call) => call.path), ['/api/auth/register', '/api/auth/login'], 'registration should be followed by one login request')
})

await runScenario('register_failure_does_not_login', async () => {
  clearAuth()
  const calls = []
  globalThis.fetch = async (path) => {
    calls.push(path)
    if (path === '/api/auth/register') {
      return jsonResponse(409, { code: 40902, data: null, message: 'Email is already registered' })
    }
    throw new Error(`Unexpected fetch path: ${path}`)
  }

  await assert.rejects(
    registerWithCredentials({ username: user.username, email: user.email, password: 'password123' }),
    /Email is already registered/,
    'registerWithCredentials should surface registration failure'
  )
  assert.deepEqual(calls, ['/api/auth/register'], 'registration failure should not call login')
  assert.equal(authToken.value, '', 'registration failure should not persist a token')
})

await runScenario('login_success_still_sets_token', async () => {
  clearAuth()
  globalThis.fetch = async (path) => {
    if (path === '/api/auth/login') {
      return jsonResponse(200, {
        code: 0,
        data: { token: 'login-token', tokenType: 'Bearer', expiresIn: 3600, user },
        message: 'Login successful',
      })
    }
    throw new Error(`Unexpected fetch path: ${path}`)
  }

  await loginWithCredentials({ email: user.email, password: 'password123' })
  assert.equal(authToken.value, 'login-token', 'loginWithCredentials should still persist login token')
  assert.equal(currentUser.value?.id, user.id, 'loginWithCredentials should still set currentUser')
})

await vite.close()
