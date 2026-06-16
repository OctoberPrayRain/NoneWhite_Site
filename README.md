# NoneWhite_Site — 空白分享站

> Galgame 分享交流社区项目 · 开发中

<p align="center">
  <img src="https://img.shields.io/badge/Status-开发中-yellow" alt="开发中" />
  <img src="https://img.shields.io/badge/Frontend-Vue3-4FC08D" alt="Vue3" />
  <img src="https://img.shields.io/badge/Backend-Rust-000000" alt="Rust" />
  <img src="https://img.shields.io/badge/Database-PostgreSQL-4169E1" alt="PostgreSQL" />
  <img src="https://img.shields.io/badge/License-MIT-yellow" alt="MIT" />
</p>

---

## 项目简介

一个 Galgame 分享站，用户可以浏览 Galgame 详情页、点赞评论、获取网盘下载链接。

**技术栈**：Vue3 + Rust + PostgreSQL  
**架构**：前后端分离

---

## 开发计划

> ✅ = 可前后端并行开发

### Phase 1 — 项目骨架（打基础）

- [x] 初始化 Vite + Vue3 前端项目
- [x] 初始化 Rust 后端项目
- [x] PostgreSQL 建库 & docker-compose.yml
- [x] 配置 .env、Rust fmt/check、gitignore
- [x] 配置 Husky pre-commit（提交前运行 Rust 检查与前端 build）
- [x] 添加后端一键启动脚本（`startBackend.sh` / `startBackend.bat`）
- [x] 添加前端一键启动脚本（`startFrontend.sh` / `startFrontend.bat`）
- [x] 配置 Vite proxy（前端请求 `/api` 和 `/uploads` 自动转发到后端）
- [x] 搭建前端基础布局（Header / Footer / 路由框架）
- [x] 约定 API 返回格式（统一 `{ code, data, message }`）
- [x] ✅ 前后端联调验证（后端 `GET /api/test`，前端 `/test-api` 已接入验证）

### Phase 2 — 用户系统（前后端可并行）

**后端：**
- [x] User Model + 建表（已添加 `users` SQL migration 与后端用户数据层基础）
- [x] 注册 / 登录 API（已通过 PostgreSQL happy path 验证 `POST /api/auth/register` / `POST /api/auth/login`）
- [x] 认证中间件（已通过 `Authorization: Bearer <token>` 访问当前用户验证）
- [x] 获取 / 更新个人资料 API（已通过 `GET/PATCH /api/users/me` 验证，资料更新仅允许 `username`）
- [x] 修改密码 API（已通过 `PATCH /api/users/me/password` 验证）
- [x] 头像上传 API（已通过 `POST /api/users/me/avatar` 验证，本地存储到 `server/uploads/avatars/`，静态 URL 为 `/uploads/avatars/...`）

**Phase 2 后端待补 / 待联调：**
- [x] 在可用 PostgreSQL 环境中执行 `server/migrations/20260605000000_create_users.sql`。
- [x] 跑通注册 → 登录 → `GET /api/users/me` → 更新用户名 → 修改密码的数据库 happy path。
- [x] 跑通头像上传 DB happy path，并验证 `/uploads/avatars/...` 静态访问。
- [x] Phase 5 决策：确认本地头像存储策略是否在通用文件上传接口中复用或升级（当前 Phase 5 已统一复用本地存储，`UPLOAD_DIR` 为基础目录，头像在 `/uploads/avatars/...`，管理员图片在 `/uploads/images/...`）。

**前端：**
- [x] 注册 / 登录页面
- [x] 退出登录
- [x] 个人中心页面（资料展示 + 编辑 + 修改密码；头像区域展示 `avatarUrl` 或首字母占位）
- [x] 前端头像上传交互（Profile 页已接入文件选择、2 MiB / PNG / JPEG / WebP 前端校验，并使用 `FormData` 字段 `avatar` 调用 `POST /api/users/me/avatar`；已在 Windows + Docker PostgreSQL 环境完成头像上传与个人中心头像回显验证）
- [x] 个人中心 — 收藏列表选项卡（Phase 2 先做 UI 占位，Phase 4 接入数据）

### Phase 3 — 游戏浏览（前后端可并行）

> 目标：先做出公开浏览，管理员功能放 Phase 5

**后端：**
- [x] Game / Category / Tag Model + 建表（已新增 `server/migrations/20260612000000_create_games.sql`，覆盖 `games` / `categories` / `tags` / `game_tags` / `screenshots`）
- [x] 游戏列表 API（分页 + 分类筛选，`GET /api/games?page=1&pageSize=12&categoryId=1&tagId=2`）
- [x] 游戏详情 API（`GET /api/games/:id`，详情内嵌 `category` / `tags` / `screenshots`）
- [x] 分类 / 标签 API（`GET /api/categories` / `GET /api/tags`）

**前端：**
- [x] 游戏列表页（卡片展示 + 分页 + 分类筛选 + 标签筛选 + URL query 状态同步）
- [x] 游戏详情页（信息展示 + 封面 + 分类/标签 + 截图轮播 + 下载/评论占位）
- [x] 游戏 API client：`client/src/api/games.js`，基于 `client/src/api/http.js`，提供 `getGames`、`getGameDetail`、`getCategories`、`getTags`
- [x] 游戏组件：`GameCard.vue`、`GameFilter.vue`、`ScreenshotCarousel.vue`
- [x] 公共状态组件：`Pagination.vue`、`BaseLoading.vue`、`EmptyState.vue`
- [x] 路由接入：`/games`、`/games/:id`；`/games` 已通过 `meta.label` 显示在 Header 导航中

**数据：**
- [x] 准备一批 seed 假数据（`server/seeds/dev_phase3_games.sql`，由 `setupDatabase.sh` / `setupDatabase.bat` 自动应用）

**Phase 3 前端状态说明：**
- [x] `npm --prefix client run build` 已通过。
- [x] `npm run lint` 已通过。
- [x] 浏览器验证 `/games`、`/games?page=1&categoryId=1&tagId=1`、`/games/1?page=1&categoryId=1&tagId=1` 可正常渲染。
- [x] 当前前端支持 mock fallback，仅作为开发兜底；真实后端接口已实现后，mock fallback 不再代表接口完成状态。
- [x] 真实后端 API 已在本机 PostgreSQL 环境通过 curl 验证：`/api/games`、`/api/games/1`、`/api/categories`。
- [x] 浏览器 `/games` 页面已通过真实后端 + PostgreSQL seed 数据联调验证；mock fallback 仅保留为接口异常时的开发兜底。

**Phase 3 后端 / 联调状态：**
- [x] `games` / `categories` / `tags` / `game_tags` / `screenshots` 表已通过 SQL migration 定义。
- [x] `GET /api/games`、`GET /api/games/:id`、`GET /api/categories`、`GET /api/tags` 已在 Rust 后端实现。
- [x] 分页参数确认为 `page` / `pageSize`。
- [x] 筛选参数确认为 `categoryId` / `tagId`。
- [x] 图片 URL 当前按数据库中存储的相对/原样字符串返回；开发 seed 暂用空字符串占位。
- [x] `screenshots` 包含在详情接口中，不新增独立截图接口。
- [x] `category` / `tags` 字段使用 `{ id, name, slug }`，与前端契约一致。
- [x] 已在本机 PostgreSQL 环境执行 Phase 3 migration + seed，并跑通真实后端 API：`GET /api/games`、`GET /api/games/:id`、`GET /api/categories`。
- [x] 浏览器 `/games` 真实数据联调已通过：Vite proxy 读取运行中的 Rust 后端与 PostgreSQL seed 数据，未使用 mock fallback。

### Phase 4 — 互动功能（前后端可并行）

**后端：**
- [x] 评论 API（发表 + 列表 + 回复 + 删除 — 管理员可删任意评论，普通用户只删自己的）
- [x] 点赞 API（点赞 / 取消点赞）
- [x] 收藏 API（收藏 / 取消收藏 / 列表）

**Phase 4 后端状态说明：**
- [x] 已新增 `server/migrations/20260613000000_create_interactions.sql`，覆盖 `comments` / `likes` / `favorites` 表、级联外键、列表/计数索引和 rollback 说明。
- [x] 已实现公开评论列表、认证评论发表/删除、认证点赞/取消、认证收藏/取消、认证个人收藏列表 API。
- [x] 评论内容后端校验：trim 后非空，最多 1000 字符；回复父评论必须属于同一游戏。
- [x] 点赞和收藏写入幂等，并刷新 `games.likes_count` / `games.favorites_count`。
- [x] 已在本机 Windows + WSL PostgreSQL 16 环境补跑 Phase 3/4 migrations + seed，并完成 Phase 4 API happy path / permission path 验证：公开评论列表返回空列表；认证评论发表与回复返回 HTTP 201；普通用户删除他人评论返回 HTTP 403；点赞/收藏重复提交保持幂等计数；`GET /api/users/me/favorites` 可读到收藏列表；取消点赞/收藏后计数归零；删除父评论后回复因 `ON DELETE CASCADE` 一并移除。

**前端：**
- [x] 评论区组件（支持回复 + 删除自己的评论）
- [x] 点赞按钮组件
- [x] 收藏按钮组件

**Phase 4 前端状态说明：**
- [x] 游戏详情页已接入点赞按钮、收藏按钮、评论列表、评论发表、回复和删除自己评论的前端交互。
- [x] 个人中心已将 Phase 2 的收藏占位切换为真实收藏列表，并接入 `GET /api/users/me/favorites?page=1&pageSize=12`。
- [x] 新增 `client/src/api/interactions.js`，统一封装评论 / 点赞 / 收藏 / 我的收藏列表接口，并复用现有 API envelope。
- [x] `npm --prefix client run build` 已通过。
- [x] 真实后端运行中的浏览器联调已通过：登录一次性用户后在 `/games/:id` 完成点赞、收藏、发表评论，并在页面中看到实时刷新结果。

### Phase 5 — 管理后台与资源

**后端：**
- [x] 管理员身份中间件（复用 Bearer token 与 `users.role=admin`，非管理员返回 HTTP 403 / `code=40301`）
- [x] 文件上传接口（图片上传 → 返回 URL，供游戏管理/头像复用；`POST /api/admin/uploads/images`）
- [x] 游戏管理 CRUD API（管理员专用 `/api/admin/games...`，保持公开 `/api/games` 兼容）
- [x] 下载链接管理 API（管理员 CRUD + 公开读取 `/api/games/{gameId}/download-links`）

**Phase 5 后端状态说明：**
- [x] 已新增 `server/migrations/20260614000000_create_download_links.sql`，覆盖 `download_links` 表、`games(id)` 级联外键和按游戏读取索引。
- [x] 已实现通用管理员图片上传：`multipart/form-data` 字段 `image`，允许 PNG/JPEG/WebP MIME + 文件签名，默认最大 5 MiB，成功返回 `data.imageUrl=/uploads/images/...`，静态读取为 `/uploads/images/{file}`。
- [x] 已实现管理员游戏创建、列表、更新、删除；创建/更新会校验分类/标签并在事务中替换 `game_tags` 与 `screenshots`。
- [x] 已实现下载链接管理员创建、列表、更新、删除和前台公开读取，响应字段为 `id, gameId, platform, url, extractCode, password, fileSize, createdAt, updatedAt`。
- [x] Live PostgreSQL Phase 5 curl 联调已通过：管理员图片上传、管理员游戏 CRUD、下载链接管理员 CRUD + 公开读取、普通用户访问管理员接口 `40301` 均通过真实 DB happy/permission path 验证。

**前端：**
- [x] 管理员后台 — 游戏管理页（增删改 + 上传封面/截图）
- [x] 管理员后台 — 下载链接管理页
- [x] 管理员后台 — 评论管理页（查看 + 删除违规评论）
- [x] 前台 — 下载区域展示（网盘链接 + 提取码）

**Phase 5 前端状态说明：**
- [x] 已新增 `/admin` 管理后台路由，管理员权限通过现有 Bearer token 与 `currentUser.role === 'admin'` 判断。
- [x] 管理后台已接入管理员游戏创建/更新/删除、封面/截图上传、下载链接 CRUD、按游戏查看并删除评论的前端交互。
- [x] 游戏详情页已接入公开下载链接读取，并展示平台、URL、提取码、密码和文件大小。
- [x] 已完成浏览器 UI 验证：`/admin` 使用 API stub 验证桌面与移动布局，`/games/1` 使用下载链接 stub 验证前台下载区域展示。
- [x] Live PostgreSQL Phase 5 前端联调已通过：`/admin` 通过真实管理员账号读取游戏、下载链接和评论资源，`/games/:id` 通过真实公开接口展示下载链接。

### Phase 6 — 搜索与部署

- [x] 搜索 API + 前端搜索页（按标题/开发商/标签，`LIKE %keyword%` + `search_text` 辅助字段）
- [x] 全局响应式适配（本轮覆盖新增搜索页、管理后台、下载区域与导航换行）
- [ ] Docker 构建镜像 + docker-compose 启动（可选备用路径；已新增构建配置并完成 Compose 静态校验，当前生产部署按用户要求不使用 Docker）
- [x] Nginx 反向代理配置（托管前端静态文件 + 转发 API 到后端）
- [x] 部署上线（直接 Rust 二进制 + systemd + Nginx + 远程 PostgreSQL）

**Phase 6 状态说明：**
- [x] 后端 `GET /api/games` 已支持 `keyword` 查询参数，服务层会 trim 空白关键词，仓储层使用 `LOWER(g.search_text) LIKE` 参与过滤。
- [x] 前端已新增 `/search` 页面，支持 URL query 同步、空关键词不请求、关键词搜索、分页和 mock fallback 提示。
- [x] 已完成浏览器联调验证：`/search?keyword=Browser%20Live` 通过真实后端返回 PostgreSQL 数据，桌面与移动端均可渲染且无横向溢出；mock fallback 仅保留为接口异常时的开发兜底。
- [x] 已新增 `.dockerignore`、`server/Dockerfile`、`client/Dockerfile`、`docker-compose.deploy.yml` 和 `deploy/nginx.conf`。
- [x] `docker compose -f docker-compose.deploy.yml config` 已通过静态校验。
- [x] `docker-compose.deploy.yml` 中的 `JWT_SECRET` / `POSTGRES_PASSWORD` 默认值仅用于本地静态校验或开发演示；真实部署前必须通过环境变量覆盖为私密值。
- [x] 当前线上部署已按用户要求改为非 Docker 路径：后端 release 二进制安装到远程 `/opt/nonewhite/releases/20260615103109`，`/opt/nonewhite/current` 指向该 release，systemd 服务 `/etc/systemd/system/nonewhite.service` active/running。
- [x] 前端 `client/dist` 已发布到远程 `/opt/nonewhite/current/public`，并通过 `/var/www/nonewhite` 提供给 Nginx 静态托管。
- [x] 远程 PostgreSQL 14.23 已在 `127.0.0.1:5432` 上创建 `nonewhite_site` / `nonewhite_user`，Phase 2/3/4/5 migrations 已应用，public schema 表数量为 10。
- [x] 远程 Nginx 1.18.0 已监听 80 端口，托管 SPA，并将 `/api/` 与 `/uploads/` 反向代理到 `127.0.0.1:3000`；UFW 默认拒绝入站导致公网 HTTP 初次失败后，已仅开放 `80/tcp`。
- [x] 远程本机 smoke test 已通过：`/api/test` 返回后端测试 JSON，`/api/games?page=1&pageSize=1` 返回 `code=0`，`/` 返回 HTTP 200 HTML；公网 `http://155.94.154.75/api/test` 返回 HTTP 200 后端测试 JSON。
- [ ] Docker 镜像实际 build 与 `docker compose up` 仍未验证；该路径当前只是备用部署方案，本轮 `docker compose -f docker-compose.deploy.yml build` 因 Docker daemon 代理 `127.0.0.1:2080` 拒绝连接，无法拉取基础镜像 metadata，未运行 `compose up`。

---

## 目录结构

```
NoneWhite_Site/
├── agent/                     # Agent 协作规则和开发计划
│   ├── AGENT_RULES.md
│   ├── COLLABORATION_PLAN.md
│   ├── JOURNALIST/            # A/B/C 角色日志与交接记录
│   └── roles/                 # A/B/C 角色详细实施文档
│       ├── README.md
│       ├── A_BACKEND_API_AUTH.md
│       ├── B_FRONTEND_PAGE_INTERACTION.md
│       └── C_DATABASE_CONTRACTS_DOCS_QA.md
│
├── client/                    # 前端 Vue3
│   └── src/
│       ├── api/               # 请求封装
│       ├── components/        # 公共组件
│       ├── router/            # 路由
│       ├── stores/            # 状态管理（如引入 Pinia 或轻量 store 时创建）
│       ├── views/             # 页面
│       ├── App.vue
│       └── main.js
│
├── server/                    # 后端 Rust
│   ├── .env.example           # 后端环境变量模板
│   ├── Cargo.toml
│   ├── migrations/            # SQL migration 文件
│   ├── seeds/                 # 本地开发 seed 数据
│   └── src/
│       ├── config.rs          # 配置
│       ├── db.rs              # PostgreSQL 连接池
│       ├── dto/               # 请求/响应 DTO
│       ├── error.rs           # API 错误码与统一错误响应
│       ├── middleware/        # 认证中间件/提取逻辑
│       ├── models/            # 数据库行模型
│       ├── repositories/      # SQL 数据访问层
│       ├── routes/            # 路由
│       ├── services/          # 业务逻辑、校验、密码哈希、JWT
│       ├── state.rs           # Axum 共享状态
│       ├── main.rs            # 服务入口
│       └── response.rs        # API 统一响应格式
│
├── docker-compose.yml          # PostgreSQL 本地开发服务
├── startBackend.sh             # Linux/macOS 后端启动脚本
├── startBackend.bat            # Windows 后端启动脚本
├── startFrontend.sh            # Linux/macOS 前端启动脚本
├── startFrontend.bat           # Windows 前端启动脚本
├── setupDatabase.sh            # Linux/macOS PostgreSQL 启动与 migration 脚本
├── setupDatabase.bat           # Windows PostgreSQL 启动与 migration 脚本
├── package.json                # 根目录脚本与 Husky 配置
└── .env.example
```

---

## 数据库设计（初版）

| 表名 | 主要字段 | 说明 |
|---|---|---|
| `users` | id, username, email, password_hash, avatar_url, role | 用户 |
| `games` | id, title, developer, publisher, release_date, description, cover_url, category_id, search_text, likes_count, favorites_count | 游戏 |
| `categories` | id, name, slug | 分类 |
| `tags` | id, name, slug | 标签 |
| `game_tags` | game_id, tag_id | 游戏-标签关联 |
| `comments` | id, user_id, game_id, content, parent_id, created_at | 评论 |
| `likes` | user_id, game_id (联合唯一) | 点赞 |
| `favorites` | user_id, game_id (联合唯一) | 收藏 |
| `download_links` | id, game_id, platform, url, extract_code, password, file_size | 下载链接 |
| `screenshots` | id, game_id, url, sort_order | 截图 |

---

## 本地启动

```bash
# 后端
# Linux/macOS
./startBackend.sh       # → localhost:3000

# 或手动启动
cd server
cp .env.example .env    # 填数据库配置
cargo run               # → localhost:3000

# Windows
startBackend.bat        # → localhost:3000

# PostgreSQL
cp .env.example .env
docker compose up -d    # 启动本地 PostgreSQL，数据保存在 Docker volume: postgres_data

# 启动 PostgreSQL 并应用 SQL migration（推荐）
# Linux/macOS
./setupDatabase.sh

# Windows
setupDatabase.bat

# 如需手动应用 migration，可在 docker compose 启动后按文件名顺序执行：
docker compose exec -T postgres psql -U nonewhite_user -d nonewhite_site < server/migrations/20260605000000_create_users.sql
docker compose exec -T postgres psql -U nonewhite_user -d nonewhite_site < server/migrations/20260612000000_create_games.sql
docker compose exec -T postgres psql -U nonewhite_user -d nonewhite_site < server/migrations/20260613000000_create_interactions.sql
docker compose exec -T postgres psql -U nonewhite_user -d nonewhite_site < server/migrations/20260614000000_create_download_links.sql
docker compose exec -T postgres psql -U nonewhite_user -d nonewhite_site < server/seeds/dev_phase3_games.sql

> Windows `setupDatabase.bat` 支持三种驱动模式：
> - `DB_SETUP_DRIVER=auto`（默认）：优先复用当前可连接的本地 PostgreSQL；若本地不可连且检测到 WSL + psql + pg_isready，则尝试在 WSL 中启动 PostgreSQL；最后才回退到 Docker
> - `DB_SETUP_DRIVER=local`：只使用当前 Windows 可访问的 PostgreSQL 服务（需安装 psql/pg_isready）
> - `DB_SETUP_DRIVER=docker`：只使用 Docker Compose（需安装 Docker Desktop）
> - `WSL_DB_DISTRO=Ubuntu-Work`：可选。如果 PostgreSQL 在非默认 WSL 发行版中，显式指定 distro 名称

# 前端（脚本会先确保依赖已安装；Vite proxy 已将 /api 和 /uploads 请求转发到后端，无需处理 CORS）
# Linux/macOS
./startFrontend.sh      # → 127.0.0.1:5173

# Windows
startFrontend.bat       # → 127.0.0.1:5173

# 或手动启动
cd client
npm install
npm run dev             # → 127.0.0.1:5173
```

> 前端 Phase 1 已完成：`client/` 已初始化，开发环境会将 `/api` 和 `/uploads` 请求代理到后端。

### 后端环境变量加载顺序

当前后端在启动阶段集中加载环境变量。入口会先加载 `server/.env`，再加载根目录 `.env` 作为项目级 fallback；`dotenvy` 默认不覆盖已存在变量，因此同名变量当前实际优先级为：shell 环境变量 > `server/.env` > 根目录 `.env` > 代码默认值。

后端启动代码通过 `CARGO_MANIFEST_DIR` 定位 `server/.env` 与根目录 `.env`，因此通过 `./startBackend.sh`、`startBackend.bat` 或 `cargo run --manifest-path server/Cargo.toml` 启动时使用相同优先级。不要提交真实 `.env`、JWT secret、数据库密码或 token。

### Phase 2 / Phase 5 上传配置

当前头像和管理员图片上传采用本地开发存储策略：

| 变量 | 默认值 | 说明 |
|---|---|---|
| `UPLOAD_DIR` | `uploads` | 相对 `server/` 的上传目录，实际头像目录为 `server/uploads/avatars/` |
| `UPLOAD_PUBLIC_BASE_URL` | `/uploads` | API 返回给前端的公开 URL 前缀 |
| `MAX_AVATAR_SIZE_BYTES` | `2097152` | 头像最大 2 MiB |
| `MAX_IMAGE_SIZE_BYTES` | `5242880` | 管理员通用图片最大 5 MiB |

`POST /api/users/me/avatar` 使用 `multipart/form-data`，字段名为 `avatar`，当前允许 `image/png`、`image/jpeg`、`image/webp`。上传成功后返回 `{ "avatarUrl": "/uploads/avatars/..." }`，并通过后端 `/uploads/avatars/{file}` 静态读取。本地上传文件已通过 `.gitignore` 排除，不要提交真实用户上传内容。

`POST /api/admin/uploads/images` 使用 `multipart/form-data`，字段名为 `image`，需要管理员 Bearer token，复用 PNG/JPEG/WebP MIME 与文件签名校验。上传成功后返回 `{ "imageUrl": "/uploads/images/..." }`，并通过 `/uploads/images/{file}` 静态读取，供游戏封面、截图和后续图片场景复用。

### Phase 2 后端 API 示例

以下成功路径需要先启动 PostgreSQL、应用 `server/migrations/20260605000000_create_users.sql`，并启动后端服务。示例只记录占位 token，不能把真实 JWT 写入 README 或协作日志。

```bash
# 注册：预期 HTTP 201，body.code=0
curl -i -X POST http://127.0.0.1:3000/api/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"username":"alice","email":"alice@example.com","password":"password123"}'

# 登录：预期 HTTP 200，body.code=0，返回 token、tokenType="Bearer"、expiresIn、user
curl -i -X POST http://127.0.0.1:3000/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","password":"password123"}'

# 当前用户：预期 HTTP 200，body.code=0
curl -i http://127.0.0.1:3000/api/users/me \
  -H 'Authorization: Bearer <token>'

# 注册无效邮箱：预期 HTTP 400，body.code=40002，message="Email is invalid"
curl -i -X POST http://127.0.0.1:3000/api/auth/register \
  -H 'Content-Type: application/json' \
  -d '{"username":"alice","email":"not-an-email","password":"password123"}'

# 登录错误密码：需先有 alice@example.com；预期 HTTP 401，body.code=40101，message="Invalid email or password"
curl -i -X POST http://127.0.0.1:3000/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"email":"alice@example.com","password":"wrong-password"}'

# 当前用户缺少 token：预期 HTTP 401，body.code=40102，message="Authentication is required"
curl -i http://127.0.0.1:3000/api/users/me

# 头像上传：需替换真实 token 和本地图片路径；预期 HTTP 200，body.code=0，data.avatarUrl 为 /uploads/avatars/...
curl -i -X POST http://127.0.0.1:3000/api/users/me/avatar \
  -H 'Authorization: Bearer <token>' \
  -F 'avatar=@./avatar.png;type=image/png'
```

> Phase 2 auth/user/avatar 数据库 happy path 已在 PostgreSQL 环境中完成验证；2026-06-10 又在 Windows + Docker Desktop + PostgreSQL 17 环境中完成注册、登录、当前用户、更新用户名、修改密码、头像上传、静态头像访问和前端个人中心头像回显回归。

### Phase 4 后端 API 示例

以下示例需要先应用 Phase 2/3/4 migrations，并启动后端。需要登录的接口只记录占位 token，不能把真实 JWT 写入 README 或协作日志。

```bash
# 评论列表：公开读取；预期 HTTP 200，body.code=0，data={ list,total,page,pageSize }
curl -i 'http://127.0.0.1:3000/api/games/1/comments?page=1&pageSize=12'

# 发表评论：预期 HTTP 201，message="Comment created successfully"
curl -i -X POST http://127.0.0.1:3000/api/games/1/comments \
  -H 'Authorization: Bearer <token>' \
  -H 'Content-Type: application/json' \
  -d '{"content":"很好玩","parentId":null}'

# 点赞 / 取消点赞：幂等，返回 liked 与 likesCount
curl -i -X POST http://127.0.0.1:3000/api/games/1/like -H 'Authorization: Bearer <token>'
curl -i -X DELETE http://127.0.0.1:3000/api/games/1/like -H 'Authorization: Bearer <token>'

# 收藏 / 取消收藏：幂等，返回 favorited 与 favoritesCount
curl -i -X POST http://127.0.0.1:3000/api/games/1/favorite -H 'Authorization: Bearer <token>'
curl -i -X DELETE http://127.0.0.1:3000/api/games/1/favorite -H 'Authorization: Bearer <token>'

# 我的收藏列表：认证读取，返回现有 GameResponse 分页结构；列表页 screenshots 可为空数组
curl -i 'http://127.0.0.1:3000/api/users/me/favorites?page=1&pageSize=12' \
  -H 'Authorization: Bearer <token>'
```

### Phase 5 后端 API 示例

以下示例需要先应用 Phase 2/3/4/5 migrations，并启动后端。管理员接口必须使用 `users.role='admin'` 的登录 token；示例只记录占位 token，不能把真实 JWT、真实网盘链接或生产密码写入 README 或协作日志。

```bash
# 管理员图片上传：预期 HTTP 200，body.code=0，data.imageUrl 为 /uploads/images/...
curl -i -X POST http://127.0.0.1:3000/api/admin/uploads/images \
  -H 'Authorization: Bearer <admin-token>' \
  -F 'image=@./cover.png;type=image/png'

# 管理员创建游戏：预期 HTTP 201，body.code=0
curl -i -X POST http://127.0.0.1:3000/api/admin/games \
  -H 'Authorization: Bearer <admin-token>' \
  -H 'Content-Type: application/json' \
  -d '{"title":"示例游戏","developer":"Example Dev","publisher":"Example Pub","releaseDate":"2026-06-12","description":"示例简介","coverUrl":"/uploads/images/cover.png","categoryId":1,"tagIds":[1,2],"screenshots":[{"url":"/uploads/images/shot.png","sortOrder":0}]}'

# 管理员列表/更新/删除游戏：列表复用现有 GameListResponse；更新请求体同创建
curl -i 'http://127.0.0.1:3000/api/admin/games?page=1&pageSize=12' \
  -H 'Authorization: Bearer <admin-token>'
curl -i -X PUT http://127.0.0.1:3000/api/admin/games/1 \
  -H 'Authorization: Bearer <admin-token>' \
  -H 'Content-Type: application/json' \
  -d '{"title":"示例游戏","developer":"Example Dev","publisher":"Example Pub","releaseDate":null,"description":"更新后的简介","coverUrl":"/uploads/images/cover.png","categoryId":1,"tagIds":[1],"screenshots":[]}'
curl -i -X DELETE http://127.0.0.1:3000/api/admin/games/1 \
  -H 'Authorization: Bearer <admin-token>'

# 下载链接：管理员 CRUD + 公开读取；示例使用占位链接，避免记录真实资源地址
curl -i -X POST http://127.0.0.1:3000/api/admin/games/1/download-links \
  -H 'Authorization: Bearer <admin-token>' \
  -H 'Content-Type: application/json' \
  -d '{"platform":"Baidu Netdisk","url":"https://example.invalid/share","extractCode":"abcd","password":"optional-password","fileSize":"1.2 GiB"}'
curl -i http://127.0.0.1:3000/api/games/1/download-links

# 普通用户访问管理员接口：预期 HTTP 403，body.code=40301，message="Permission denied"
curl -i 'http://127.0.0.1:3000/api/admin/games?page=1&pageSize=12' \
  -H 'Authorization: Bearer <user-token>'
```

### 开发检查

```bash
npm run lint            # 运行 Rust fmt/check 与前端 build
```

提交前会通过 Husky 自动执行 `npm run lint`。

---

## 团队分工

三人或多 Agent 并行开发前，必须先阅读 [`agent/COLLABORATION_PLAN.md`](agent/COLLABORATION_PLAN.md)，按其中的角色所有权、变量命名、API 契约、数据库字段映射和交接规则执行。

| 角色 | 负责内容 |
|---|---|
| 前端 | 页面开发、组件封装、前后端联调 |
| 后端 | API 开发、数据库、认证鉴权 |
| UI/设计 | 界面风格、交互设计 |
| 文档/测试 | API 文档、功能测试 |

---

## License

[MIT](LICENSE)
