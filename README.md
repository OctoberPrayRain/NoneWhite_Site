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
- [x] 配置 Vite proxy（前端请求 `/api` 自动转发到后端）
- [x] 搭建前端基础布局（Header / Footer / 路由框架）
- [x] 约定 API 返回格式（统一 `{ code, data, message }`）
- [x] ✅ 前后端联调验证（后端 `GET /api/test`，前端 `/test-api` 已接入验证）

### Phase 2 — 用户系统（前后端可并行）

**后端：**
- [x] User Model + 建表（已添加 `users` SQL migration 与后端用户数据层基础）
- [x] 注册 / 登录 API（代码已实现 `POST /api/auth/register` / `POST /api/auth/login`；DB happy path 待 PostgreSQL 环境联调）
- [x] 认证中间件（代码已实现 `Authorization: Bearer <token>` 校验；DB 用户查询 happy path 待联调）
- [x] 获取 / 更新个人资料 API（代码已实现 `GET/PATCH /api/users/me`，资料更新仅允许 `username`；DB happy path 待联调）
- [x] 修改密码 API（代码已实现 `PATCH /api/users/me/password`；DB happy path 待联调）
- [ ] 头像上传 API（缺：存储策略、上传目录/对象存储、URL 形式、大小限制、文件类型白名单）

**Phase 2 后端待补 / 待联调：**
- [ ] 在可用 PostgreSQL 环境中执行 `server/migrations/20260605000000_create_users.sql`。
- [ ] 跑通注册 → 登录 → `GET /api/users/me` → 更新用户名 → 修改密码的数据库 happy path。
- [ ] 确认头像上传策略后实现头像上传 API，并决定是否复用 Phase 5 文件上传接口。

**前端：**
- [ ] 注册 / 登录页面
- [ ] 退出登录
- [ ] 个人中心页面（资料展示 + 编辑 + 修改密码 + 头像上传）
- [ ] 个人中心 — 收藏列表选项卡（Phase 2 先做 UI 占位，Phase 4 接入数据）

### Phase 3 — 游戏浏览（前后端可并行）

> 目标：先做出公开浏览，管理员功能放 Phase 5

**后端：**
- [ ] Game / Category / Tag Model + 建表
- [ ] 游戏列表 API（分页 + 分类筛选）
- [ ] 游戏详情 API
- [ ] 分类 / 标签 API

**前端：**
- [ ] 游戏列表页（卡片展示 + 分页 + 侧栏筛选）
- [ ] 游戏详情页（信息展示 + 封面 + 截图轮播）

**数据：**
- [ ] 准备一批 seed 假数据（方便联调）

### Phase 4 — 互动功能（前后端可并行）

**后端：**
- [ ] 评论 API（发表 + 列表 + 回复 + 删除 — 管理员可删任意评论，普通用户只删自己的）
- [ ] 点赞 API（点赞 / 取消点赞）
- [ ] 收藏 API（收藏 / 取消收藏 / 列表）

**前端：**
- [ ] 评论区组件（支持回复 + 删除自己的评论）
- [ ] 点赞按钮组件
- [ ] 收藏按钮组件

### Phase 5 — 管理后台与资源

**后端：**
- [ ] 管理员身份中间件
- [ ] 文件上传接口（图片上传 → 返回 URL，供游戏管理/头像复用）
- [ ] 游戏管理 CRUD API
- [ ] 下载链接管理 API

**前端：**
- [ ] 管理员后台 — 游戏管理页（增删改 + 上传封面/截图）
- [ ] 管理员后台 — 下载链接管理页
- [ ] 管理员后台 — 评论管理页（查看 + 删除违规评论）
- [ ] 前台 — 下载区域展示（网盘链接 + 提取码）

### Phase 6 — 搜索与部署

- [ ] 搜索 API + 前端搜索页（按标题/开发商/标签，`LIKE %keyword%` + `search_text` 辅助字段）
- [ ] 全局响应式适配
- [ ] Docker 构建镜像 + docker-compose 启动
- [ ] Nginx 反向代理配置（托管前端静态文件 + 转发 API 到后端）
- [ ] 部署上线

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

# 应用 SQL migration（当前项目使用 SQL 文件，尚未引入 migration 工具）
set -a && . ./.env && set +a
psql "$DATABASE_URL" -f server/migrations/20260605000000_create_users.sql

# 前端（脚本会先确保依赖已安装；Vite proxy 已将 /api 请求转发到后端，无需处理 CORS）
# Linux/macOS
./startFrontend.sh      # → 127.0.0.1:5173

# Windows
startFrontend.bat       # → 127.0.0.1:5173

# 或手动启动
cd client
npm install
npm run dev             # → 127.0.0.1:5173
```

> 前端 Phase 1 已完成：`client/` 已初始化，开发环境会将 `/api` 请求代理到后端。

### 后端环境变量加载顺序

当前后端在启动阶段集中加载环境变量。通过 `./startBackend.sh` 或手动 `cd server && cargo run` 启动时，入口会先尝试加载根目录 `.env`，再尝试加载 `server/.env`；`dotenvy` 默认不覆盖已存在变量，因此同名变量当前实际优先级为：shell 环境变量 > 根目录 `.env` > `server/.env` > 代码默认值。

协作契约推荐后续以后端 `server/.env` 作为主要后端配置来源；在代码优先级调整前，如两个文件存在同名变量，请以当前实际加载顺序为准，避免在两个 `.env` 中写入冲突值。不要提交真实 `.env`、JWT secret、数据库密码或 token。

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
```

> 头像上传 API 仍待确认存储策略（上传目录、URL 形式、大小限制、文件类型）后实现。
> 当前环境可以解析 `docker compose config`，但没有可用 Docker daemon / `psql`，因此 Phase 2 auth/user API 已完成代码、编译、单元测试和非 DB curl 回归；注册/登录/资料/改密的数据库 happy path 仍需在具备 PostgreSQL 的环境中补充联调记录。

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
