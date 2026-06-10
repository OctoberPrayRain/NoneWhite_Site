import { ApiError, requestJson } from './http'

const DEFAULT_PAGE = 1
const DEFAULT_PAGE_SIZE = 12

const MOCK_CATEGORIES = [
  { id: 1, name: '剧情', slug: 'story' },
  { id: 2, name: '悬疑', slug: 'mystery' },
  { id: 3, name: '恋爱', slug: 'romance' },
  { id: 4, name: '奇幻', slug: 'fantasy' },
]

const MOCK_TAGS = [
  { id: 1, name: '校园', slug: 'school' },
  { id: 2, name: '治愈', slug: 'healing' },
  { id: 3, name: '多结局', slug: 'multi-ending' },
  { id: 4, name: '悬疑', slug: 'suspense' },
  { id: 5, name: '幻想', slug: 'fantasy' },
]

const MOCK_GAMES = [
  {
    id: 1,
    title: '雨后第七封信',
    developer: 'NoneWhite Studio',
    publisher: 'NoneWhite',
    release_date: '2024-01-18',
    description: '一段发生在旧教学楼里的短篇视觉小说，围绕失物、信件与毕业前夜展开。',
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
    title: '白塔回声',
    developer: 'Amber Loop',
    publisher: 'NoneWhite',
    release_date: '2024-04-02',
    description: '探索封闭白塔中的记忆房间，拼合角色之间被抹去的约定。',
    category: MOCK_CATEGORIES[1],
    tags: [MOCK_TAGS[3], MOCK_TAGS[2]],
    likes_count: 92,
    favorites_count: 31,
    screenshots: [{ id: 201, url: '', sort_order: 1 }],
  },
  {
    id: 3,
    title: '薄荷色夏日',
    developer: 'Mint Days',
    publisher: 'Indie Harbor',
    release_date: '2023-08-12',
    description: '以社团活动为主线的轻恋爱故事，包含多条角色支线与夏日祭章节。',
    category: MOCK_CATEGORIES[2],
    tags: [MOCK_TAGS[0], MOCK_TAGS[1], MOCK_TAGS[2]],
    likes_count: 210,
    favorites_count: 83,
    screenshots: [],
  },
  {
    id: 4,
    title: '星屑档案室',
    developer: 'Orbit Type',
    publisher: 'NoneWhite',
    release_date: '2025-02-26',
    description: '在漂浮档案馆中检索星球文明的最后记录，选择会影响档案修复顺序。',
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
    title: '午后三点的侦探',
    developer: 'Clockwork Note',
    publisher: 'Indie Harbor',
    release_date: '2023-11-09',
    description: '小镇咖啡馆里的日常推理合集，每个案件都藏着角色关系的新线索。',
    category: MOCK_CATEGORIES[1],
    tags: [MOCK_TAGS[3], MOCK_TAGS[1]],
    likes_count: 154,
    favorites_count: 55,
    screenshots: [{ id: 501, url: '', sort_order: 1 }],
  },
  {
    id: 6,
    title: '月台尽头',
    developer: 'Silent Rail',
    publisher: 'NoneWhite',
    release_date: '2024-09-21',
    description: '末班车停靠在不存在的月台，玩家需要在循环中找回乘客的名字。',
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

function normalizeGame(game) {
  return {
    id: game.id,
    title: game.title ?? '未命名游戏',
    developer: game.developer ?? '未知开发商',
    publisher: game.publisher ?? '未知发行商',
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

  const filteredGames = MOCK_GAMES.filter((game) => {
    const matchCategory = !categoryId || String(game.category?.id) === categoryId
    const matchTag = !tagId || game.tags.some((tag) => String(tag.id) === tagId)
    return matchCategory && matchTag
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
      throw new ApiError('游戏不存在或暂时无法加载', {
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
