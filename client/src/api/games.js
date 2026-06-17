import { ApiError, createAuthHeaders, requestJson } from './http'

const DEFAULT_PAGE = 1
const DEFAULT_PAGE_SIZE = 12

const MOCK_CATEGORIES = [
  { id: 1, name: '文档', slug: 'document' },
  { id: 2, name: '表格', slug: 'spreadsheet' },
  { id: 3, name: '素材', slug: 'asset' },
  { id: 4, name: '教程', slug: 'tutorial' },
]

const MOCK_TAGS = [
  { id: 1, name: '模板', slug: 'template' },
  { id: 2, name: '说明', slug: 'guide' },
  { id: 3, name: '多格式', slug: 'multi-format' },
  { id: 4, name: '归档', slug: 'archive' },
  { id: 5, name: '示例', slug: 'sample' },
]

const MOCK_GAMES = [
  {
    id: 1,
    title: '项目交付说明模板',
    developer: 'NoneWhite Studio',
    publisher: 'NoneWhite',
    release_date: '2024-01-18',
    description: '面向项目归档的说明文档，包含交付清单、版本记录和验收备注。',
    category: MOCK_CATEGORIES[0],
    tags: [MOCK_TAGS[0], MOCK_TAGS[1], MOCK_TAGS[2]],
    likes_count: 128,
    favorites_count: 46,
    screenshots: [
      { id: 101, url: '', sort_order: 1 },
      { id: 102, url: '', sort_order: 2 },
    ],
  },
  {
    id: 2,
    title: '季度数据整理表',
    developer: 'Amber Loop',
    publisher: 'NoneWhite',
    release_date: '2024-04-02',
    description: '用于整理季度数据的表格文件，内含分类字段、汇总页和备注模板。',
    category: MOCK_CATEGORIES[1],
    tags: [MOCK_TAGS[3], MOCK_TAGS[2]],
    likes_count: 92,
    favorites_count: 31,
    screenshots: [{ id: 201, url: '', sort_order: 1 }],
  },
  {
    id: 3,
    title: '品牌素材打包清单',
    developer: 'Mint Days',
    publisher: 'Indie Harbor',
    release_date: '2023-08-12',
    description: '汇总标识、配色、图像和授权说明的素材包索引，便于团队统一取用。',
    category: MOCK_CATEGORIES[2],
    tags: [MOCK_TAGS[0], MOCK_TAGS[1], MOCK_TAGS[2]],
    likes_count: 210,
    favorites_count: 83,
    screenshots: [],
  },
  {
    id: 4,
    title: '归档目录示例',
    developer: 'Orbit Type',
    publisher: 'NoneWhite',
    release_date: '2025-02-26',
    description: '展示多级目录、命名规则和文件说明的归档示例，适合快速复用。',
    category: MOCK_CATEGORIES[3],
    tags: [MOCK_TAGS[4], MOCK_TAGS[2]],
    likes_count: 76,
    favorites_count: 29,
    screenshots: [
      { id: 401, url: '', sort_order: 1 },
      { id: 402, url: '', sort_order: 2 },
      { id: 403, url: '', sort_order: 3 },
    ],
  },
  {
    id: 5,
    title: '会议纪要合集',
    developer: 'Clockwork Note',
    publisher: 'Indie Harbor',
    release_date: '2023-11-09',
    description: '按主题整理的会议纪要合集，包含行动项、负责人和后续跟进记录。',
    category: MOCK_CATEGORIES[1],
    tags: [MOCK_TAGS[3], MOCK_TAGS[1]],
    likes_count: 154,
    favorites_count: 55,
    screenshots: [{ id: 501, url: '', sort_order: 1 }],
  },
  {
    id: 6,
    title: '下载资源说明书',
    developer: 'Silent Rail',
    publisher: 'NoneWhite',
    release_date: '2024-09-21',
    description: '记录资源下载方式、校验信息和使用注意事项，帮助用户快速确认文件内容。',
    category: MOCK_CATEGORIES[0],
    tags: [MOCK_TAGS[3], MOCK_TAGS[4]],
    likes_count: 188,
    favorites_count: 67,
    screenshots: [{ id: 601, url: '', sort_order: 1 }],
  },
]

function parsePositiveInt(value, fallback) {
  const parsed = Number.parseInt(value, 10)
  return Number.isFinite(parsed) && parsed > 0 ? parsed : fallback
}

function normalizeCategory(category) {
  if (!category) {
    return null
  }

  return {
    id: category.id,
    name: category.name ?? '',
    slug: category.slug ?? '',
  }
}

function normalizeTag(tag) {
  return {
    id: tag.id,
    name: tag.name ?? '',
    slug: tag.slug ?? '',
  }
}

function normalizeScreenshot(screenshot) {
  return {
    id: screenshot.id,
    url: screenshot.url ?? '',
    sortOrder: screenshot.sortOrder ?? screenshot.sort_order ?? 0,
  }
}

export function normalizeGame(game) {
  return {
    id: game.id,
    title: game.title ?? '未命名文件',
    developer: game.developer ?? '未知提供方',
    publisher: game.publisher ?? '未知发布方',
    releaseDate: game.releaseDate ?? game.release_date ?? '',
    description: game.description ?? '',
    coverUrl: game.coverUrl ?? game.cover_url ?? '',
    category: normalizeCategory(game.category),
    tags: Array.isArray(game.tags) ? game.tags.map(normalizeTag) : [],
    likesCount: Number(game.likesCount ?? game.likes_count ?? 0),
    favoritesCount: Number(game.favoritesCount ?? game.favorites_count ?? 0),
    screenshots: Array.isArray(game.screenshots)
      ? game.screenshots.map(normalizeScreenshot).sort((a, b) => a.sortOrder - b.sortOrder)
      : [],
    approvalStatus: game.approvalStatus ?? game.approval_status ?? 'approved',
    submittedByUserId: game.submittedByUserId ?? game.submitted_by_user_id ?? null,
    reviewedByUserId: game.reviewedByUserId ?? game.reviewed_by_user_id ?? null,
    reviewedAt: game.reviewedAt ?? game.reviewed_at ?? null,
  }
}

function createMockReason(error) {
  return error instanceof Error ? error.message : '接口暂不可用，已使用前端 mock fallback'
}

function getMockGames(params = {}, mockReason = '') {
  const page = parsePositiveInt(params.page, DEFAULT_PAGE)
  const pageSize = parsePositiveInt(params.pageSize, DEFAULT_PAGE_SIZE)
  const categoryId = params.categoryId ? String(params.categoryId) : ''
  const tagId = params.tagId ? String(params.tagId) : ''
  const keyword = String(params.keyword ?? '').trim().toLowerCase()

  const filteredGames = MOCK_GAMES.filter((game) => {
    const matchCategory = !categoryId || String(game.category?.id) === categoryId
    const matchTag = !tagId || game.tags.some((tag) => String(tag.id) === tagId)
    const searchText = [
      game.title,
      game.developer,
      game.publisher,
      game.description,
      game.category?.name,
      ...game.tags.map((tag) => tag.name),
    ]
      .filter(Boolean)
      .join(' ')
      .toLowerCase()
    const matchKeyword = !keyword || searchText.includes(keyword)
    return matchCategory && matchTag && matchKeyword
  })

  const start = (page - 1) * pageSize
  const list = filteredGames.slice(start, start + pageSize).map(normalizeGame)

  return {
    list,
    total: filteredGames.length,
    page,
    pageSize,
    isMock: true,
    mockReason,
  }
}

function buildGameQuery(params = {}) {
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

// Frontend expected contract:
// GET /api/games?page=1&pageSize=12&categoryId=1&tagId=2
// { code: 0, data: { list: [], total: 0, page: 1, pageSize: 12 }, message: 'success' }
export async function getGames(params = {}) {
  const query = buildGameQuery(params)

  try {
    const data = await requestJson(`/api/games?${query}`)
    const page = parsePositiveInt(data?.page, params.page ?? DEFAULT_PAGE)
    const pageSize = parsePositiveInt(data?.pageSize ?? data?.page_size, params.pageSize ?? DEFAULT_PAGE_SIZE)

    return {
      list: Array.isArray(data?.list) ? data.list.map(normalizeGame) : [],
      total: Number(data?.total ?? 0),
      page,
      pageSize,
      isMock: false,
      mockReason: '',
    }
  } catch (error) {
    return getMockGames(params, createMockReason(error))
  }
}

// Frontend expected contract:
// GET /api/games/:id
// { code: 0, data: { id, title, release_date, cover_url, category, tags, likes_count, favorites_count, screenshots }, message: 'success' }
export async function getGameDetail(id) {
  try {
    const data = await requestJson(`/api/games/${id}`)
    return {
      game: normalizeGame(data),
      isMock: false,
      mockReason: '',
    }
  } catch (error) {
    const game = MOCK_GAMES.find((item) => String(item.id) === String(id))

    if (!game) {
      throw new ApiError('文件不存在或暂时无法加载', {
        status: 404,
        data: {
          isMockFallback: true,
          originalMessage: createMockReason(error),
        },
      })
    }

    return {
      game: normalizeGame(game),
      isMock: true,
      mockReason: createMockReason(error),
    }
  }
}

// Frontend expected contract:
// GET /api/categories
// { code: 0, data: [{ id, name, slug }], message: 'success' }
export async function getCategories() {
  try {
    const data = await requestJson('/api/categories')
    return {
      list: Array.isArray(data) ? data.map(normalizeCategory).filter(Boolean) : [],
      isMock: false,
      mockReason: '',
    }
  } catch (error) {
    return {
      list: MOCK_CATEGORIES.map(normalizeCategory),
      isMock: true,
      mockReason: createMockReason(error),
    }
  }
}

// Frontend expected contract:
// GET /api/tags
// { code: 0, data: [{ id, name, slug }], message: 'success' }
export async function getTags() {
  try {
    const data = await requestJson('/api/tags')
    return {
      list: Array.isArray(data) ? data.map(normalizeTag) : [],
      isMock: false,
      mockReason: '',
    }
  } catch (error) {
    return {
      list: MOCK_TAGS.map(normalizeTag),
      isMock: true,
      mockReason: createMockReason(error),
    }
  }
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

// Frontend expected contract:
// GET /api/games/:id/download-links
// { code: 0, data: [{ id, gameId, platform, url, extractCode, password, fileSize }], message: 'success' }
export async function getPublicDownloadLinks(id) {
  const data = await requestJson(`/api/games/${id}/download-links`)
  return Array.isArray(data) ? data.map(normalizeDownloadLink) : []
}

export async function submitGameSubmission(payload, authToken) {
  const data = await requestJson('/api/games/submissions', {
    method: 'POST',
    headers: {
      'Content-Type': 'application/json',
      ...createAuthHeaders(authToken),
    },
    body: JSON.stringify(payload),
  })
  return normalizeGame(data)
}
