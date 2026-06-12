import { normalizeGame } from './games'
import { createAuthHeaders, requestJson } from './http'

const DEFAULT_PAGE = 1
const DEFAULT_PAGE_SIZE = 12

function parsePositiveInt(value, fallback) {
  const parsed = Number.parseInt(value, 10)
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback
}

function buildPaginationQuery(params = {}) {
  const searchParams = new URLSearchParams()
  searchParams.set('page', parsePositiveInt(params.page, DEFAULT_PAGE))
  searchParams.set('pageSize', parsePositiveInt(params.pageSize, DEFAULT_PAGE_SIZE))
  return searchParams.toString()
}

function normalizeComment(comment) {
  return {
    id: Number(comment?.id ?? 0),
    userId: Number(comment?.userId ?? comment?.user_id ?? 0),
    username: comment?.username ?? '匿名用户',
    avatarUrl: comment?.avatarUrl ?? comment?.avatar_url ?? '',
    gameId: Number(comment?.gameId ?? comment?.game_id ?? 0),
    content: comment?.content ?? '',
    parentId: comment?.parentId ?? comment?.parent_id ?? null,
    createdAt: comment?.createdAt ?? comment?.created_at ?? '',
  }
}

function normalizeLikeStatus(data) {
  return {
    liked: Boolean(data?.liked),
    likesCount: Number(data?.likesCount ?? data?.likes_count ?? 0),
  }
}

function normalizeFavoriteStatus(data) {
  return {
    favorited: Boolean(data?.favorited),
    favoritesCount: Number(data?.favoritesCount ?? data?.favorites_count ?? 0),
  }
}

export async function getComments(gameId, params = {}) {
  const query = buildPaginationQuery(params)
  const data = await requestJson(`/api/games/${gameId}/comments?${query}`)

  return {
    list: Array.isArray(data?.list) ? data.list.map(normalizeComment) : [],
    total: Number(data?.total ?? 0),
    page: parsePositiveInt(data?.page, params.page ?? DEFAULT_PAGE),
    pageSize: parsePositiveInt(data?.pageSize ?? data?.page_size, params.pageSize ?? DEFAULT_PAGE_SIZE),
  }
}

export async function createComment(gameId, { content, parentId }, authToken) {
  const data = await requestJson(`/api/games/${gameId}/comments`, {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify({
      content,
      parentId: parentId ?? null,
    }),
  })

  return normalizeComment(data)
}

export function deleteComment(commentId, authToken) {
  return requestJson(`/api/comments/${commentId}`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })
}

export async function likeGame(gameId, authToken) {
  const data = await requestJson(`/api/games/${gameId}/like`, {
    method: 'POST',
    headers: createAuthHeaders(authToken),
  })

  return normalizeLikeStatus(data)
}

export async function unlikeGame(gameId, authToken) {
  const data = await requestJson(`/api/games/${gameId}/like`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })

  return normalizeLikeStatus(data)
}

export async function favoriteGame(gameId, authToken) {
  const data = await requestJson(`/api/games/${gameId}/favorite`, {
    method: 'POST',
    headers: createAuthHeaders(authToken),
  })

  return normalizeFavoriteStatus(data)
}

export async function unfavoriteGame(gameId, authToken) {
  const data = await requestJson(`/api/games/${gameId}/favorite`, {
    method: 'DELETE',
    headers: createAuthHeaders(authToken),
  })

  return normalizeFavoriteStatus(data)
}

export async function getMyFavorites(params = {}, authToken) {
  const query = buildPaginationQuery(params)
  const data = await requestJson(`/api/users/me/favorites?${query}`, {
    headers: createAuthHeaders(authToken),
  })

  return {
    list: Array.isArray(data?.list) ? data.list.map(normalizeGame) : [],
    total: Number(data?.total ?? 0),
    page: parsePositiveInt(data?.page, params.page ?? DEFAULT_PAGE),
    pageSize: parsePositiveInt(data?.pageSize ?? data?.page_size, params.pageSize ?? DEFAULT_PAGE_SIZE),
  }
}
