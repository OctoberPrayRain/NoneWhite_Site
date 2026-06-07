import { requestJson } from './http'

export function register({ username, email, password }) {
  return requestJson('/api/auth/register', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ username, email, password }),
  })
}

export function login({ email, password }) {
  return requestJson('/api/auth/login', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
    },
    body: JSON.stringify({ email, password }),
  })
}

export function logoutLocal() {
  return true
}
