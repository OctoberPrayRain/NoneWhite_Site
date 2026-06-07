export class ApiError extends Error {
  constructor(message, details = {}) {
    super(message)
    this.name = 'ApiError'
    this.status = details.status
    this.code = details.code
    this.data = details.data
  }
}

export function createAuthHeaders(authToken) {
  return authToken ? { Authorization: `Bearer ${authToken}` } : {}
}

export async function requestJson(path, options = {}) {
  const response = await fetch(path, options)
  const body = await response.json()
  const hasEnvelope = body && typeof body === 'object' && 'code' in body

  if (!response.ok || !hasEnvelope || body.code !== 0) {
    throw new ApiError(body?.message || `请求失败：${response.status}`, {
      status: response.status,
      code: hasEnvelope ? body.code : undefined,
      data: hasEnvelope ? body.data : null,
    })
  }

  return body.data
}
