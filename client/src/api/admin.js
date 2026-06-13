import { normalizeGame } from './games'
import { createAuthHeaders, requestJson } from './http'

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

  return searchParams.toString()
}

function normalizeDownloadLink(downloadLink) {
  return {
    id: Number(downloadLink?.id ?? 0),
    gameId: Number(downloadLink?.gameId ?? downloadLink?.game_id ?? 0),
    platform: downloadLink?.platform ?? '',
    url: downloadLink?.url ?? '',
    extractCode: downloadLink?.extractCode ?? downloadLink?.extract_code ?? '',
    password: downloadLink?.password ?? '',
    fileSize: downloadLink?.fileSize ?? downloadLink?.file_size ?? '',
    createdAt: downloadLink?.createdAt ?? downloadLink?.created_at ?? '',
    updatedAt: downloadLink?.updatedAt ?? downloadLink?.updated_at ?? '',
  }
}

function createJsonOptions(method, payload, authToken) {
  return {
    method,
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify(payload),
  }
}

export async function getAdminGames(params = {}, authToken) {
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

export async function createAdminGame(payload, authToken) {
  const data = await requestJson('/api/admin/games', createJsonOptions('POST', payload, authToken))
  return normalizeGame(data)
}

export async function updateAdminGame(gameId, payload, authToken) {
  const data = await requestJson(
    `/api/admin/games/${gameId}`,
    createJsonOptions('PUT', payload, authToken),
  )
  return normalizeGame(data)
}

export function deleteAdminGame(gameId, authToken) {
  return requestJson(`/api/admin/games/${gameId}`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })
}

export async function uploadAdminImage(file, authToken) {
  const formData = new FormData()
  formData.append('image', file)

  const data = await requestJson('/api/admin/uploads/images', {
    method: 'POST',
    headers: createAuthHeaders(authToken),
    body: formData,
  })

  return {
    imageUrl: data?.imageUrl ?? data?.image_url ?? '',
  }
}

export async function getAdminDownloadLinks(gameId, authToken) {
  const data = await requestJson(`/api/admin/games/${gameId}/download-links`, {
    headers: createAuthHeaders(authToken),
  })

  return Array.isArray(data) ? data.map(normalizeDownloadLink) : []
}

export async function createAdminDownloadLink(gameId, payload, authToken) {
  const data = await requestJson(
    `/api/admin/games/${gameId}/download-links`,
    createJsonOptions('POST', payload, authToken),
  )
  return normalizeDownloadLink(data)
}

export async function updateAdminDownloadLink(gameId, downloadLinkId, payload, authToken) {
  const data = await requestJson(
    `/api/admin/games/${gameId}/download-links/${downloadLinkId}`,
    createJsonOptions('PUT', payload, authToken),
  )
  return normalizeDownloadLink(data)
}

export function deleteAdminDownloadLink(gameId, downloadLinkId, authToken) {
  return requestJson(`/api/admin/games/${gameId}/download-links/${downloadLinkId}`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })
}

export async function getPublicDownloadLinks(gameId) {
  const data = await requestJson(`/api/games/${gameId}/download-links`)
  return Array.isArray(data) ? data.map(normalizeDownloadLink) : []
}
