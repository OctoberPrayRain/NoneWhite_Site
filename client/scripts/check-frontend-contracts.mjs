#!/usr/bin/env node

/**
 * Frontend contract checker – zero-dependency static analysis.
 * Reads router/index.js as text, scans views/components for banned phrases.
 * Exit code 1 on any failure.
 */

import fs from 'node:fs'
import path from 'node:path'
import process from 'node:process'

const CLIENT = path.resolve(path.dirname(new URL(import.meta.url).pathname), '..')
const ROUTER = path.join(CLIENT, 'src', 'router', 'index.js')
const STYLE = path.join(CLIENT, 'src', 'style.css')
const API_GAMES = path.join(CLIENT, 'src', 'api', 'games.js')
const API_ADMIN = path.join(CLIENT, 'src', 'api', 'admin.js')
const ADMIN_VIEW = path.join(CLIENT, 'src', 'views', 'AdminConsoleView.vue')
const GAME_DETAIL_VIEW = path.join(CLIENT, 'src', 'views', 'game', 'GameDetailView.vue')
const VIEWS_DIR = path.join(CLIENT, 'src', 'views')
const COMPONENTS_DIR = path.join(CLIENT, 'src', 'components')

const failures = []
function fail(msg) { failures.push(msg) }
function pass(msg) { console.log(`  \x1b[32m✓\x1b[0m ${msg}`) }
function err(msg) { console.log(`  \x1b[31m✗\x1b[0m ${msg}`) }

/* ------------------------------------------------------------------ */
/*  1. Router file must exist and be readable                         */
/* ------------------------------------------------------------------ */
let routerSrc
try {
  routerSrc = fs.readFileSync(ROUTER, 'utf8')
  pass(`Router file readable: ${path.relative(CLIENT, ROUTER)}`)
} catch {
  err(`Cannot read router file: ${ROUTER}`)
  console.log('\n\x1b[31mFATAL\x1b[0m: router file missing')
  process.exit(1)
}

let styleSrc = ''
try {
  styleSrc = fs.readFileSync(STYLE, 'utf8')
  pass(`Style file readable: ${path.relative(CLIENT, STYLE)}`)
} catch {
  err(`Cannot read style file: ${STYLE}`)
  fail('Missing global style file')
}

let gamesApiSrc = ''
try {
  gamesApiSrc = fs.readFileSync(API_GAMES, 'utf8')
  pass(`Games API file readable: ${path.relative(CLIENT, API_GAMES)}`)
} catch {
  err(`Cannot read games API file: ${API_GAMES}`)
  fail('Missing games API file')
}

let adminApiSrc = ''
try {
  adminApiSrc = fs.readFileSync(API_ADMIN, 'utf8')
  pass(`Admin API file readable: ${path.relative(CLIENT, API_ADMIN)}`)
} catch {
  err(`Cannot read admin API file: ${API_ADMIN}`)
  fail('Missing admin API file')
}

let adminViewSrc = ''
try {
  adminViewSrc = fs.readFileSync(ADMIN_VIEW, 'utf8')
  pass(`Admin view readable: ${path.relative(CLIENT, ADMIN_VIEW)}`)
} catch {
  err(`Cannot read admin view: ${ADMIN_VIEW}`)
  fail('Missing admin view')
}

let gameDetailViewSrc = ''
try {
  gameDetailViewSrc = fs.readFileSync(GAME_DETAIL_VIEW, 'utf8')
  pass(`Game detail view readable: ${path.relative(CLIENT, GAME_DETAIL_VIEW)}`)
} catch {
  err(`Cannot read game detail view: ${GAME_DETAIL_VIEW}`)
  fail('Missing game detail view')
}

/* ------------------------------------------------------------------ */
/*  2. Route existence checks                                         */
/* ------------------------------------------------------------------ */
console.log('\n--- Route existence ---')

const requiredPaths = [
  '/',
  '/test-api',
  '/login',
  '/register',
  '/profile',
  '/files',
  '/files/:id',
  '/submit-file',
  '/search',
  '/admin',
]

for (const rp of requiredPaths) {
  // Match path: '/foo' or path: '/foo/:id' with optional whitespace
  const re = new RegExp(`path\\s*:\\s*['"\`]\\s*${escapeRegExp(rp)}\\s*['"\`]`)
  if (re.test(routerSrc)) {
    pass(`Route ${rp} exists`)
  } else {
    err(`Route ${rp} NOT found`)
    fail(`Missing route: ${rp}`)
  }
}

/* ------------------------------------------------------------------ */
/*  3. Meta flag checks on route blocks                               */
/* ------------------------------------------------------------------ */
console.log('\n--- Route meta flags ---')

function findRouteBlock(sourcePath) {
  const re = new RegExp(
    `\\{\\s*\\n\\s*path\\s*:\\s*['"\`]\\s*${escapeRegExp(sourcePath)}\\s*['"\`]`
  )
  const match = sourcePath === '/files/:id'
    ? /{\s*\n\s*path\s*:\s*['"`]\s*\/files\/:id\s*['"`]/
    : re
  const idx = match.exec(routerSrc)
  if (!idx) return null
  // Grab from match start to next route or end-of-array
  const start = idx.index
  const rest = routerSrc.slice(start)
  // Find the closing of this route object by counting braces
  let depth = 0
  for (let i = 0; i < rest.length; i++) {
    if (rest[i] === '{') depth++
    if (rest[i] === '}') { depth--; if (depth === 0) return rest.slice(0, i + 1) }
  }
  return rest
}

const authRoutes = ['/profile', '/files', '/files/:id', '/submit-file', '/search']
for (const rp of authRoutes) {
  const block = findRouteBlock(rp)
  if (!block) {
    err(`Cannot locate route block for ${rp}`)
    fail(`Route block not found: ${rp}`)
    continue
  }
  if (/requiresAuth\s*:\s*true/.test(block)) {
    pass(`${rp} has requiresAuth: true`)
  } else {
    err(`${rp} missing requiresAuth: true`)
    fail(`Missing requiresAuth: true on ${rp}`)
  }
}

// Admin needs both
{
  const block = findRouteBlock('/admin')
  if (!block) {
    err('Cannot locate route block for /admin')
    fail('Route block not found: /admin')
  } else {
    const hasAuth = /requiresAuth\s*:\s*true/.test(block)
    const hasAdmin = /requiresAdmin\s*:\s*true/.test(block)
    if (hasAuth && hasAdmin) {
      pass('/admin has requiresAuth: true and requiresAdmin: true')
    } else {
      if (!hasAuth) { err('/admin missing requiresAuth: true'); fail('Missing requiresAuth: true on /admin') }
      if (!hasAdmin) { err('/admin missing requiresAdmin: true'); fail('Missing requiresAdmin: true on /admin') }
    }
  }
}

// Guest-only routes
const guestRoutes = ['/login', '/register']
for (const rp of guestRoutes) {
  const block = findRouteBlock(rp)
  if (!block) {
    err(`Cannot locate route block for ${rp}`)
    fail(`Route block not found: ${rp}`)
    continue
  }
  if (/guestOnly\s*:\s*true/.test(block)) {
    pass(`${rp} has guestOnly: true`)
  } else {
    err(`${rp} missing guestOnly: true`)
    fail(`Missing guestOnly: true on ${rp}`)
  }
}

/* ------------------------------------------------------------------ */
/*  4. /test-api must NOT expose a normal nav label                   */
/* ------------------------------------------------------------------ */
console.log('\n--- /test-api nav exposure ---')
{
  const block = findRouteBlock('/test-api')
  if (!block) {
    err('Cannot locate route block for /test-api')
    fail('Route block not found: /test-api')
  } else {
    // A "normal" label is a non-empty string in meta.label
    const labelMatch = /label\s*:\s*['"`]([^'"`]+)['"`]/.exec(block)
    if (labelMatch) {
      err(`/test-api has nav label '${labelMatch[1]}' — must not expose one`)
      fail('/test-api exposes a nav label')
    } else {
      pass('/test-api does not expose a nav label')
    }
  }
}

/* ------------------------------------------------------------------ */
/*  5. Router guard redirect pattern                                  */
/* ------------------------------------------------------------------ */
console.log('\n--- Router guard ---')
{
  // Must redirect unauthenticated protected routes to /login with redirect: to.fullPath
  if (/return\s*\{\s*\n?\s*path\s*:\s*['"`]\/login['"`]\s*,?\s*\n?\s*query\s*:\s*\{[^}]*redirect\s*:\s*to\.fullPath/.test(routerSrc)) {
    pass('Guard redirects to /login with redirect: to.fullPath')
  } else {
    err('Guard does not match expected redirect pattern')
    fail('Router guard missing /login redirect with redirect: to.fullPath')
  }
}

/* ------------------------------------------------------------------ */
/*  6. Banned scaffold / dev phrases in views & components            */
/* ------------------------------------------------------------------ */
console.log('\n--- Banned scaffold phrases ---')

const BANNED = [
  'Phase 1',
  'Phase 2',
  'Phase 3',
  'Phase 4',
  'Phase 5',
  'Phase 6',
  'Project Skeleton',
  'Vue 文档',
  'frontend shell',
  'POST /api/auth/login',
  'nonewhite_auth_token',
  'GET /api/test',
  'Vite dev server',
]

function scanDir(dir) {
  if (!fs.existsSync(dir)) return []
  const entries = fs.readdirSync(dir, { withFileTypes: true })
  const files = []
  for (const e of entries) {
    const full = path.join(dir, e.name)
    if (e.isDirectory()) files.push(...scanDir(full))
    else if (e.name.endsWith('.vue') || e.name.endsWith('.js') || e.name.endsWith('.ts'))
      files.push(full)
  }
  return files
}

const sourceFiles = [...scanDir(VIEWS_DIR), ...scanDir(COMPONENTS_DIR)]
let phraseHits = 0

for (const file of sourceFiles) {
  const rel = path.relative(CLIENT, file)
  const content = fs.readFileSync(file, 'utf8')
  for (const phrase of BANNED) {
    if (content.includes(phrase)) {
      err(`Banned phrase "${phrase}" found in ${rel}`)
      fail(`Banned phrase "${phrase}" in ${rel}`)
      phraseHits++
    }
  }
}

if (phraseHits === 0) {
  pass('No banned scaffold phrases found')
}

/* ------------------------------------------------------------------ */
/*  7. Game submission moderation contracts                           */
/* ------------------------------------------------------------------ */
console.log('\n--- Submission moderation contracts ---')

if (/export\s+async\s+function\s+submitGameSubmission\s*\(/.test(gamesApiSrc)) {
  pass('Games API exports submitGameSubmission')
} else {
  err('Games API missing submitGameSubmission')
  fail('Missing submitGameSubmission API')
}

if (/\/api\/games\/submissions/.test(gamesApiSrc)) {
  pass('Games API posts to /api/games/submissions')
} else {
  err('Games API does not call /api/games/submissions')
  fail('Missing user game submission endpoint')
}

if (/export\s+async\s+function\s+adminGetPendingGames\s*\(/.test(adminApiSrc)) {
  pass('Admin API exports adminGetPendingGames')
} else {
  err('Admin API missing adminGetPendingGames')
  fail('Missing pending game admin API')
}

if (/export\s+async\s+function\s+adminApproveGame\s*\(/.test(adminApiSrc)) {
  pass('Admin API exports adminApproveGame')
} else {
  err('Admin API missing adminApproveGame')
  fail('Missing approve game admin API')
}

if (/\/api\/admin\/games\/pending/.test(adminApiSrc)) {
  pass('Admin API calls pending games endpoint')
} else {
  err('Admin API does not call /api/admin/games/pending')
  fail('Missing pending games endpoint call')
}

if (/\/api\/admin\/games\/\$\{[^}]+\}\/approve/.test(adminApiSrc)) {
  pass('Admin API calls approve endpoint')
} else {
  err('Admin API does not call approve endpoint')
  fail('Missing approve endpoint call')
}

if (/待审核/.test(adminViewSrc) && /adminApproveGame/.test(adminViewSrc)) {
  pass('Admin view exposes pending approval workflow')
} else {
  err('Admin view missing pending approval workflow')
  fail('Missing admin pending approval UI')
}

/* ------------------------------------------------------------------ */
/*  8. Download link privacy contracts                                */
/* ------------------------------------------------------------------ */
console.log('\n--- Download link privacy contracts ---')

if (/\{\{\s*link\.url\s*\}\}/.test(gameDetailViewSrc)) {
  err('Game detail download card renders link.url as visible text')
  fail('Download card must not expose raw download URL as visible text')
} else {
  pass('Download card does not render raw download URL as visible text')
}

if (/:href=\"link\.url\"/.test(gameDetailViewSrc)) {
  pass('Download card keeps link.url only as the click target')
} else {
  err('Game detail download card missing link.url click target')
  fail('Download card must keep a functional download target')
}

if (/\/uploads\/resources/.test(gameDetailViewSrc) || /\{\{\s*link\.url\s*\}\}/.test(gameDetailViewSrc)) {
   err('Game detail exposes raw upload resource marker')
   fail('Game detail exposes raw upload resource marker')
} else {
   pass('Game detail does not expose raw resource markers')
}

/* ------------------------------------------------------------------ */
/*  9. Upload APIs and Forms contracts                                */
/* ------------------------------------------------------------------ */
console.log('\n--- Upload APIs and Forms contracts ---')

if (/export\s+async\s+function\s+uploadUserImage\s*\(/.test(gamesApiSrc)) {
  pass('Games API exports uploadUserImage')
} else {
  err('Games API missing uploadUserImage')
  fail('Missing uploadUserImage API')
}

if (/export\s+async\s+function\s+uploadUserResource\s*\(/.test(gamesApiSrc)) {
  pass('Games API exports uploadUserResource')
} else {
  err('Games API missing uploadUserResource')
  fail('Missing uploadUserResource API')
}

let submitViewSrc = ''
try {
  submitViewSrc = fs.readFileSync(path.join(CLIENT, 'src', 'views', 'SubmitGameView.vue'), 'utf8')
  if (/downloadLinks\s*:/.test(submitViewSrc)) {
    pass('Submit payload includes downloadLinks')
  } else {
    err('Submit payload missing downloadLinks')
    fail('Submit payload missing downloadLinks')
  }

  if (/\{\{\s*link\.url\s*\}\}/.test(submitViewSrc)) {
    err('SubmitGameView exposes link.url directly in template')
    fail('SubmitGameView exposes link.url directly in template')
  } else {
    pass('SubmitGameView does not expose raw link.url in template')
  }

  if (/formatFileSize/.test(submitViewSrc)) {
    pass('SubmitGameView uses formatFileSize for resource sizes')
  } else {
    err('SubmitGameView missing formatFileSize usage')
    fail('SubmitGameView missing formatFileSize usage')
  }
} catch {
  err('Could not read SubmitGameView.vue for upload checks')
  fail('Missing SubmitGameView.vue')
}

if (/uploadResource|uploadAdminResource|uploadUserResource/.test(adminViewSrc)) {
    pass('Admin resource upload is wired in AdminConsoleView.vue')
} else {
    err('Admin resource upload not wired')
    fail('Admin resource upload not wired')
}

if (/formatFileSize/.test(adminViewSrc)) {
  pass('AdminConsoleView uses formatFileSize for resource sizes')
} else {
  err('AdminConsoleView missing formatFileSize usage')
  fail('AdminConsoleView missing formatFileSize usage')
}

/* ------------------------------------------------------------------ */
/*  10. Visual system contract                                         */
/* ------------------------------------------------------------------ */
console.log('\n--- Visual system contract ---')

const VISUAL_REQUIREMENTS = [
  ['Midnight backdrop token', /--bg-night\s*:/],
  ['Warm gold accent token', /--accent-gold\s*:/],
  ['Cyan glow accent token', /--accent-cyan\s*:/],
  ['Ambient background layer', /body::before/],
  ['Surface entry animation', /@keyframes\s+surfaceRise/],
  ['Feature card hover lift', /\.feature-card:hover[\s\S]*transform\s*:\s*translateY/],
  ['Reduced motion safety', /@media\s*\(prefers-reduced-motion:\s*reduce\)/],
]

for (const [label, pattern] of VISUAL_REQUIREMENTS) {
  if (pattern.test(styleSrc)) {
    pass(label)
  } else {
    err(`${label} missing from ${path.relative(CLIENT, STYLE)}`)
    fail(`Visual system missing: ${label}`)
  }
}

/* ------------------------------------------------------------------ */
/*  Summary                                                           */
/* ------------------------------------------------------------------ */
console.log('')
if (failures.length > 0) {
  console.log(`\x1b[31mCONTRACT CHECK FAILED\x1b[0m — ${failures.length} failure(s):\n`)
  for (const f of failures) console.log(`  · ${f}`)
  process.exit(1)
} else {
  console.log('\x1b[32mALL CONTRACT CHECKS PASSED\x1b[0m')
  process.exit(0)
}

/* ------------------------------------------------------------------ */
/*  Helpers                                                           */
/* ------------------------------------------------------------------ */
function escapeRegExp(s) {
  return s.replace(/[.*+?^${}()|[\]\\]/g, '\\$&')
}
