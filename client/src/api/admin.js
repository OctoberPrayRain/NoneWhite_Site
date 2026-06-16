import { ApiError, createAuthHeaders, requestJson } from './http'
import { normalizeGame } from './games'

const DEFAULT_PAGE = 1
const DEFAULT_PAGE_SIZE = 12

function parsePositiveInt(value, fallback) {
  const parsed = Number.parseInt(value, 10)
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback
}

function buildAdminGameQuery(params = {}) {
  const searchParams = new URLSearchParams()
  searchParams.set('page', parsePositiveInt(params.page, DEFAULT_PAGE))
  searchParams.set('pageSize', parsePositiveInt(params.pageSize, DEFAULT_PAGE_SIZE))

  if (params.categoryId) {
    searchParams.set('categoryId', params.categoryId)
  }

  if (params.tagId) {
    searchParams.set('tagId', params.tagId)
  }

  const keyword = String(params.keyword ?? '').trim()
  if (keyword) {
    searchParams.set('keyword', keyword)
  }

  return searchParams.toString()
}

export function normalizeDownloadLink(link) {
  return {
    id: Number(link?.id ?? 0),
    gameId: Number(link?.gameId ?? link?.game_id ?? 0),
    platform: link?.platform ?? '',
    url: link?.url ?? '',
    extractCode: link?.extractCode ?? link?.extract_code ?? '',
    password: link?.password ?? '',
    fileSize: link?.fileSize ?? link?.file_size ?? '',
    createdAt: link?.createdAt ?? link?.created_at ?? '',
    updatedAt: link?.updatedAt ?? link?.updated_at ?? '',
  }
}

export async function adminGetGames(params = {}, authToken) {
  const query = buildAdminGameQuery(params)
  const data = await requestJson(`/api/admin/games?${query}`, {
    headers: createAuthHeaders(authToken),
  })

  return {
    list: Array.isArray(data?.list) ? data.list.map(normalizeGame) : [],
    total: Number(data?.total ?? 0),
    page: parsePositiveInt(data?.page, params.page ?? DEFAULT_PAGE),
    pageSize: parsePositiveInt(data?.pageSize ?? data?.page_size, params.pageSize ?? DEFAULT_PAGE_SIZE),
  }
}

export async function adminGetPendingGames(params = {}, authToken) {
  const query = buildAdminGameQuery(params)
  const data = await requestJson(`/api/admin/games/pending?${query}`, {
    headers: createAuthHeaders(authToken),
  })

  return {
    list: Array.isArray(data?.list) ? data.list.map(normalizeGame) : [],
    total: Number(data?.total ?? 0),
    page: parsePositiveInt(data?.page, params.page ?? DEFAULT_PAGE),
    pageSize: parsePositiveInt(data?.pageSize ?? data?.page_size, params.pageSize ?? DEFAULT_PAGE_SIZE),
  }
}

export async function adminApproveGame(id, authToken) {
  const data = await requestJson(`/api/admin/games/${id}/approve`, {
    method: 'POST',
    headers: createAuthHeaders(authToken),
  })
  return normalizeGame(data)
}

export async function adminCreateGame(data, authToken) {
  return requestJson('/api/admin/games', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify(data),
  })
}

export async function adminUpdateGame(id, data, authToken) {
  return requestJson(`/api/admin/games/${id}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify(data),
  })
}

export async function adminDeleteGame(id, authToken) {
  return requestJson(`/api/admin/games/${id}`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })
}

export async function uploadImage(file, authToken) {
  const formData = new FormData()
  formData.append('image', file)

  const response = await fetch('/api/admin/uploads/images', {
    method: 'POST',
    headers: createAuthHeaders(authToken),
    body: formData,
  })
  const text = await response.text()
  let body = null

  if (text) {
    try {
      body = JSON.parse(text)
    } catch {
      throw new ApiError('接口返回格式不是有效 JSON', {
        status: response.status,
        data: text,
      })
    }
  }

  const hasEnvelope = body && typeof body === 'object' && 'code' in body
  if (!response.ok || !hasEnvelope || body.code !== 0) {
    throw new ApiError(hasEnvelope ? body.message : `请求失败：${response.status}`, {
      status: response.status,
      code: hasEnvelope ? body.code : undefined,
      data: hasEnvelope ? body.data : null,
    })
  }

  return body.data
}

export async function adminGetDownloadLinks(gameId, authToken) {
  const data = await requestJson(`/api/admin/games/${gameId}/download-links`, {
    headers: createAuthHeaders(authToken),
  })

  return Array.isArray(data) ? data.map(normalizeDownloadLink) : []
}

export async function adminCreateDownloadLink(gameId, data, authToken) {
  const response = await requestJson(`/api/admin/games/${gameId}/download-links`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify(data),
  })

  return normalizeDownloadLink(response)
}

export async function adminUpdateDownloadLink(gameId, linkId, data, authToken) {
  const response = await requestJson(`/api/admin/games/${gameId}/download-links/${linkId}`, {
    method: 'PUT',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify(data),
  })

  return normalizeDownloadLink(response)
}

export async function adminDeleteDownloadLink(gameId, linkId, authToken) {
  return requestJson(`/api/admin/games/${gameId}/download-links/${linkId}`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })
}
