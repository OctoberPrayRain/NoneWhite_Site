import { createAuthHeaders, requestJson } from './http'

export function fetchCurrentUser(authToken) {
  return requestJson('/api/users/me', {
    headers: createAuthHeaders(authToken),
  })
}

export function updateCurrentUser({ username }, authToken) {
  return requestJson('/api/users/me', {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify({ username }),
  })
}

export function changePassword({ currentPassword, newPassword }, authToken) {
  return requestJson('/api/users/me/password', {
    method: 'PATCH',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify({ currentPassword, newPassword }),
  })
}
