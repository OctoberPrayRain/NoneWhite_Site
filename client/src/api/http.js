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

async function parseResponseBody(response) {
  const text = await response.text()

  if (!text) {
    return null
  }

  try {
    return JSON.parse(text)
  } catch {
    throw new ApiError('接口返回格式不是有效 JSON', {
      status: response.status,
      data: text,
    })
  }
}

export async function requestJson(path, options = {}) {
  const { returnEnvelope = false, ...fetchOptions } = options
  const response = await fetch(path, fetchOptions)
  const body = await parseResponseBody(response)
  const hasEnvelope = body && typeof body === 'object' && 'code' in body

  if (!response.ok || !hasEnvelope || body.code !== 0) {
    const message = hasEnvelope ? body.message : `请求失败：${response.status}`

    throw new ApiError(message || '接口返回格式不符合约定', {
      status: response.status,
      code: hasEnvelope ? body.code : undefined,
      data: hasEnvelope ? body.data : null,
    })
  }

  return returnEnvelope ? body : body.data
}
