# NoneWhite_Site 三人协作实现计划

> 本文档是 `NoneWhite_Site` 的协作开发入口文档。任何人类开发者或 Agent 在开始写代码前，必须先阅读 `README.md`、`agent/AGENT_RULES.md` 和本文档，再根据自己的角色读取对应源码。

---

## 1. 文档目标

本计划用于解决三个人并行开发时最容易出现的问题：变量命名不一致、接口字段不一致、数据库字段和前端字段对不上、多人同时改同一个文件、README 状态误标、接口实现和页面实现脱节。

本文档只定义协作方式、实现顺序、变量规范、接口契约、验证规则和交接规则，不新增 `README.md` 之外的产品范围。任何功能是否要做，以 `README.md` 的 Phase 计划为准；任何实际实现状态，以当前仓库文件和验证结果为准。

当前项目基线（2026-06-15 阶段性状态）：

- 前端：Vue3 + Vite + Vue Router，位于 `client/`。
- 后端：Rust 2021 + axum，位于 `server/`。
- 数据库：PostgreSQL；`docker-compose.yml` 默认使用 PostgreSQL 17，Phase 2 DB happy path 使用 WSL PostgreSQL 16.14 验证，本轮 Phase 3/4/5 live QA 使用本机已有 Docker PostgreSQL 15 镜像验证；线上服务器使用远程 PostgreSQL 14.23。
- 已实现：Phase 1 项目骨架；Phase 2 用户系统（含头像本地上传）；Phase 3 游戏浏览（含真实后端浏览器联调）；Phase 4 互动功能（前后端已接入并通过 live 浏览器联调）；Phase 5 管理后台与资源（含管理员图片本地上传、live curl 与 live 前端联调）；Phase 6 搜索、响应式适配和直接二进制线上部署（Rust release binary + systemd + Nginx + 远程 PostgreSQL）。
- 未完成/待接入：Phase 6 Docker 镜像 build/up 仅保留为可选备用路径，当前仍受 Docker daemon 代理 `127.0.0.1:2080` 阻断；线上部署已按用户要求使用非 Docker 路径完成。

---

## 2. 开工前必须阅读

每个 Agent 接到任务后，必须按顺序阅读：

1. `README.md`
2. `agent/AGENT_RULES.md`
3. `agent/COLLABORATION_PLAN.md`
4. `agent/roles/README.md`
5. 自己角色对应的详细实施文档：A 读 `agent/roles/A_BACKEND_API_AUTH.md`，B 读 `agent/roles/B_FRONTEND_PAGE_INTERACTION.md`，C 读 `agent/roles/C_DATABASE_CONTRACTS_DOCS_QA.md`
6. 自己角色对应的日志文档：A 读 `agent/JOURNALIST/A_BACKEND_API_AUTH/A_BACKEND_API_AUTH_LOG.md`，B 读 `agent/JOURNALIST/B_FRONTEND_PAGE_INTERACTION/B_FRONTEND_PAGE_INTERACTION_LOG.md`，C 读 `agent/JOURNALIST/C_DATABASE_CONTRACTS_DOCS_QA/C_DATABASE_CONTRACTS_DOCS_QA_LOG.md`
7. 自己负责区域的源码
8. 如果任务涉及接口、数据库或跨端字段，还必须阅读本文档中的“统一命名规范”“API 契约”“字段映射表”“环境变量契约”

开工前必须确认：

- 当前 Git 分支和工作区状态。
- README 中的 Phase 状态是否和实际文件一致。
- 自己要改的文件是否属于自己的角色所有权范围。
- 是否需要先和其他角色确认接口、字段、数据库 schema。
- 是否会修改共享高冲突文件。

如果 README 和实际代码不一致，以实际代码为准，并在完成任务后修正 README。不要凭记忆判断项目状态。

---

## 3. 三人角色划分

三个人应按下面三个角色并行开发。每个角色可以由人类或 Agent 执行。

| 角色 | 名称 | 主要职责 | 主要目录 |
|---|---|---|---|
| A | 后端/API/认证负责人 | Rust API、业务逻辑、认证鉴权、后端测试、后端依赖 | `server/` |
| B | 前端/页面/交互负责人 | Vue 页面、组件、路由、前端 API client、表单交互、前端构建 | `client/` |
| C | 数据库/契约/文档/QA 负责人 | schema、migration、seed、接口契约、字段映射、README、联调检查 | 根目录文档、数据库文件、共享契约 |

### 3.1 角色 A：后端/API/认证负责人

职责：

- 设计并实现后端路由、handler、service、repository 或等价模块。
- 复用 `server/src/response.rs` 的统一返回格式。
- 所有接口路径必须以 `/api` 开头。
- 所有请求和响应 DTO 必须符合本文档的字段映射表。
- 后端是输入校验的最终可信来源。
- 密码、token、权限、数据库错误处理由后端负责。
- 新增后端依赖时，必须说明用途，并确保 `cargo check` 通过。

不能做：

- 不能随意改前端字段名来适配后端临时字段。
- 不能返回未记录在契约中的 JSON 字段。
- 不能把 `password`、`password_hash`、JWT secret 或数据库错误细节返回给前端。
- 不能在没有 schema 契约时先写大量业务代码。

### 3.2 角色 B：前端/页面/交互负责人

职责：

- 维护 Vue Router、页面、组件、前端 API 封装。
- 所有 API 请求必须通过 `client/src/api/` 下的封装函数发出。
- 前端字段名必须使用 API 契约中的 JSON 字段名。
- 前端校验用于改善体验，但不能替代后端校验。
- 页面状态必须区分 loading、success、empty、error。
- 新增页面必须接入路由和基础布局。

不能做：

- 不能硬编码后端地址，开发环境使用相对路径 `/api/...` 和 Vite proxy。
- 不能自己发明接口字段或错误格式。
- 不能把 mock 数据当成真实接口完成状态。
- 不能修改后端接口返回格式来适配临时 UI。

### 3.3 角色 C：数据库/契约/文档/QA 负责人

职责：

- 在写代码前维护接口契约、字段映射表、数据库表结构说明。
- 负责 migration 或 schema 文件的组织方式，一旦选定不能混用。
- 维护 README Phase 状态、目录结构、本地启动说明和验证命令。
- 负责跨角色联调验收，确认前后端字段一致。
- 维护本文档中新增约定。

不能做：

- 不能把未完成的任务标记为 `[x]`。
- 不能在没有代码和验证的情况下更新 README 说“已完成”。
- 不能随意修改 A/B 的实现文件。
- 不能在契约中写不存在的接口、命令或路径。

---

## 4. 目录所有权和共享文件规则

### 4.1 后端所有权

角色 A 默认拥有：

```txt
server/Cargo.toml
server/Cargo.lock
server/src/**
server/.env.example
```

角色 A 可以创建的推荐目录：

```txt
server/src/routes/
server/src/models/
server/src/dto/
server/src/services/
server/src/repositories/
server/src/middleware/
server/src/error.rs
server/tests/
```

新增目录前必须保持简单，不要一次性创建空架构。只有当前任务需要时才创建。

### 4.2 前端所有权

角色 B 默认拥有：

```txt
client/package.json
client/package-lock.json
client/vite.config.js
client/src/api/**
client/src/components/**
client/src/router/**
client/src/stores/**
client/src/views/**
client/src/App.vue
client/src/main.js
client/src/style.css
client/public/**
```

角色 B 可以创建的推荐目录：

```txt
client/src/api/
client/src/components/auth/
client/src/components/games/
client/src/stores/
client/src/views/
client/src/utils/
```

如果引入 Pinia，必须先在 `client/package.json` 增加依赖，并在 README 中记录；不要只创建 `stores/` 空目录。

### 4.3 数据库和文档所有权

角色 C 默认拥有：

```txt
README.md
agent/AGENT_RULES.md
agent/COLLABORATION_PLAN.md
.env.example
docker-compose.yml
```

角色 C 可以创建的推荐目录，具体名称必须先定一个，不要混用：

```txt
server/migrations/
server/seeds/
docs/api/
docs/database/
```

### 4.4 共享高冲突文件

以下文件容易产生冲突，修改前必须确认是否真的需要：

```txt
README.md
agent/AGENT_RULES.md
agent/COLLABORATION_PLAN.md
package.json
client/package.json
client/package-lock.json
client/src/router/index.js
server/Cargo.toml
server/Cargo.lock
server/src/routes/mod.rs
docker-compose.yml
.env.example
server/.env.example
```

共享文件修改规则：

- 只改和当前任务直接相关的最小内容。
- 不做全文件格式化。
- 修改后在交接说明中列出文件名和原因。
- 如果两个角色都需要修改同一个共享文件，由角色 C 先更新契约或文档，再由 A/B 按契约实现。

### 4.5 代码重叠区域处理规则

有些文件不是某一个角色独占，而是多个角色都会间接依赖。此类文件必须写清“谁先改、谁复核、谁消费”，否则三个人并行时容易互相覆盖。处理原则：契约先于实现，入口文件最后修改，交接必须写明影响面。

| 重叠区域 | 主责 | 协作角色 | 修改顺序 | 禁止事项 | 交接必须说明 |
|---|---|---|---|---|---|
| `server/src/routes/mod.rs` | A | C | C 确认 API 契约后，A 创建 handler，再注册 route | 不写业务逻辑；不删除 `/api/test`；不重排无关路由 | 新增 method/path、handler 文件、测试命令 |
| `client/src/router/index.js` | B | C | C 确认页面和权限语义后，B 新增 view，再注册 route | 不删除现有 `/`、`/test-api`；不让未完成页面出现在完成状态 | 新增 path/name/meta、是否影响 Header 导航 |
| `client/src/components/AppHeader.vue` | B | C | 路由 meta 确认后再调整导航过滤 | 不在多个页面重复写导航逻辑；不硬编码未实现页面状态 | 登录/未登录时显示哪些入口 |
| `server/src/response.rs` | A | C | C 确认 envelope/error code 后，A 调整 response/error | 不为单个接口创建特殊 envelope；不暴露数据库原始错误 | envelope 是否变更、影响哪些接口 |
| `client/src/api/http.js` | B | A/C | C 确认 envelope，A 确认错误格式，B 实现统一请求层 | 不在各 API 文件重复解析 envelope；不吞掉后端 `message` | 错误处理策略、token header 策略 |
| DTO/字段映射表 | C | A/B | C 先改字段映射，A 改 Rust DTO，B 改前端变量 | 不临时改字段名适配单端；不返回未记录字段 | DB/Rust/API/frontend 字段变化 |
| `.env.example` 和 `server/.env.example` | C | A | A 提出新增变量，C 同步示例和说明，A 再读取 | 不提交真实 secret；不让两个示例冲突 | 新变量名、默认值、是否必填 |
| `server/Cargo.toml` / `Cargo.lock` | A | C | A 说明依赖用途，C 确认不影响范围，A 修改并验证 | 不一次性加入未使用依赖；不跳过 `cargo check` | 新依赖、用途、验证结果 |
| `client/package.json` / `package-lock.json` | B | C | B 说明依赖用途，C 确认是否属于当前 Phase，B 修改并 build | 不加入 UI/状态库后不使用；不只改 lockfile | 新依赖、用途、build 结果 |
| `README.md` Phase checklist | C | A/B | A/B 提供验证结果，C 再更新状态 | 不因“代码准备”勾选完成；不把 mock 当联调 | 勾选项、验证命令、仍未完成项 |
| `config.rs` / `db.rs` / `AppState` | A | C | C 确认 env 契约，A 实现配置和状态注入 | 不在多个文件重复读取 env；不每请求创建连接池 | 配置来源、必填变量、启动验证 |

如果一个任务同时触碰两个以上重叠区域，必须在开工前写出本次修改顺序。例如“先更新契约字段表，再改后端 DTO，最后改前端 API client”。

---

## 5. 分支和提交协作规则

推荐每个角色使用独立分支：

```txt
feat/backend-auth
feat/frontend-auth
feat/db-user-schema
docs/collaboration-plan
fix/backend-api-response
```

规则：

- 不要直接提交到 `development`，除非项目负责人明确要求。
- 不要 amend 或 force push，除非项目负责人明确要求。
- 不要 revert 其他人的代码，除非项目负责人明确要求。
- 每个提交只做一个清晰目标。
- 不要把无关的前端、后端、文档、格式化混在一个提交里。
- 如果一次提交是垂直切片，必须同时包含契约、实现、测试和文档更新。

提交信息使用 `agent/AGENT_RULES.md` 中的 Conventional Commits 风格：

```txt
type(scope): 中文简述
```

示例：

```txt
docs(collaboration): 添加三人协作实现计划
feat(db): 添加用户表 migration
feat(backend): 实现用户注册接口
feat(frontend): 添加登录注册页面
docs(readme): 更新用户系统阶段进度
```

---

## 6. Phase 2 推荐并行顺序

Phase 2 是用户系统，最容易出现前后端字段不一致，因此必须契约先行。

### 6.1 Wave 0：契约锁定

负责人：角色 C，A/B 必须参与确认。

产出：

- 用户表字段最终名称。
- 用户 API endpoint 列表。
- request/response DTO。
- 错误码和错误 message。
- 前端页面和接口的对应关系。
- 环境变量新增项。

没有完成 Wave 0，A/B 不要开始正式编码，只能做不依赖接口的布局或测试准备。

### 6.2 Wave 1：数据库和后端基础

角色 C：

- 创建用户表 migration 或 schema 文件。
- 记录字段映射表。
- 更新 `.env.example` 和 `server/.env.example` 中必要变量。

角色 A：

- 选择并接入数据库访问方式。
- 建立用户模型和 DTO。
- 编写注册、登录、鉴权相关测试。

角色 B：

- 创建前端 auth API client 文件。
- 创建登录/注册页面骨架。
- 按契约写表单字段，不连接未完成接口时可使用契约样例数据。

### 6.3 Wave 2：注册和登录闭环

角色 A：

- 实现 `POST /api/auth/register`。
- 实现 `POST /api/auth/login`。
- 生成和校验 JWT。
- 返回统一 API envelope。

角色 B：

- 接入注册和登录页面。
- 处理 loading、error、success。
- 保存认证状态。

角色 C：

- 用 curl 或等价方式验证接口。
- 检查前端字段和后端响应字段一致。
- 更新接口文档。

### 6.4 Wave 3：个人资料和退出登录

角色 A：

- 实现认证中间件。
- 实现 `GET /api/users/me`。
- 实现 `PATCH /api/users/me`。
- 实现 `PATCH /api/users/me/password`。

角色 B：

- 实现个人中心页面。
- 实现退出登录。
- 实现资料编辑和修改密码 UI。

角色 C：

- 验证 token 缺失、token 无效、密码错误、更新成功等场景。
- 更新 README Phase 状态。

### 6.5 Wave 4：头像上传

头像上传依赖存储策略，必须先由 A/C 明确：

- 文件保存在哪里。
- 返回 URL 是相对路径还是完整 URL。
- 最大文件大小。
- 允许的文件类型。
- 是否需要后端静态文件服务。

当前 Phase 2 已确认本地开发策略：后端保存到 `server/uploads/avatars/`，API 返回 `/uploads/avatars/...`，最大 2 MiB，允许 `image/png`、`image/jpeg`、`image/webp`，并由后端提供 `/uploads/avatars/{file_name}` 静态读取。B 尚未接入上传交互时只能保留 UI 占位，不能标记前端头像上传完成。

---

## 7. 全项目统一命名规范

### 7.1 数据库命名

数据库统一使用 `snake_case`。

| 类型 | 规则 | 示例 |
|---|---|---|
| 表名 | 复数 `snake_case` | `users`, `games`, `download_links` |
| 主键 | 固定 `id` | `id` |
| 外键 | `{entity}_id` | `user_id`, `game_id`, `category_id` |
| 时间字段 | `{action}_at` | `created_at`, `updated_at`, `release_date` |
| 计数字段 | `{thing}_count` | `likes_count`, `favorites_count` |
| URL 字段 | `{name}_url` | `avatar_url`, `cover_url` |
| 哈希字段 | `{name}_hash` | `password_hash` |

禁止混用：

- `avatar` / `avatar_url` / `avatarUrl` 在同一层混用。
- `userID` / `user_id` / `uid` 随意混用。
- `create_time` 和 `created_at` 混用。

### 7.2 Rust 后端命名

Rust 遵守 Rust 常规命名：

| 类型 | 规则 | 示例 |
|---|---|---|
| 文件名 | `snake_case.rs` | `auth.rs`, `user_profile.rs` |
| 模块名 | `snake_case` | `auth`, `user_profile` |
| 变量/函数 | `snake_case` | `user_id`, `get_current_user` |
| struct/enum | `PascalCase` | `RegisterRequest`, `UserResponse` |
| 常量 | `SCREAMING_SNAKE_CASE` | `DEFAULT_PAGE_SIZE` |

DTO 命名规则：

```txt
{Action}{Entity}Request
{Entity}Response
{Action}{Entity}Response
```

示例：

```txt
RegisterRequest
LoginRequest
AuthTokenResponse
UserProfileResponse
UpdateUserProfileRequest
ChangePasswordRequest
```

### 7.3 前端命名

前端当前是 JavaScript，不是 TypeScript。

| 类型 | 规则 | 示例 |
|---|---|---|
| Vue 页面 | `PascalCase.vue` | `LoginView.vue`, `ProfileView.vue` |
| Vue 组件 | `PascalCase.vue` | `AuthForm.vue`, `GameCard.vue` |
| API 文件 | 功能名小写 | `auth.js`, `users.js`, `games.js` |
| 变量/函数 | `camelCase` | `userId`, `fetchCurrentUser` |
| 路由 path | `kebab-case` | `/test-api`, `/profile`, `/games/:id` |
| 路由 name | `kebab-case` | `test-api`, `profile`, `game-detail` |
| CSS class | `kebab-case` | `auth-card`, `profile-form` |

前端状态命名：

```js
const status = ref('idle')
const errorMessage = ref('')
const currentUser = ref(null)
```

状态字符串统一优先使用：

```txt
idle
loading
success
empty
error
```

### 7.4 API JSON 命名

API 对外 JSON 字段统一使用 `camelCase`，数据库和 Rust 内部可以使用 `snake_case`。

示例：

| 数据库字段 | Rust 字段 | API JSON 字段 | 前端变量 |
|---|---|---|---|
| `avatar_url` | `avatar_url` | `avatarUrl` | `avatarUrl` |
| `created_at` | `created_at` | `createdAt` | `createdAt` |
| `password_hash` | `password_hash` | 不返回 | 不保存 |

后端负责从 DB/Rust 内部字段映射到 API JSON 字段。前端只能使用 API JSON 字段，不直接使用数据库字段名。

---

## 8. 统一 API 返回格式

所有 API 都必须返回统一 envelope：

```json
{
  "code": 0,
  "data": {},
  "message": "Success"
}
```

规则：

- `code = 0` 表示业务成功。
- `code != 0` 表示业务失败。
- HTTP status 仍然必须正确，例如参数错误用 400，未登录用 401，权限不足用 403，资源不存在用 404。
- `message` 给前端显示或调试使用，必须简短明确。
- `data` 的类型必须在契约中写清楚，不能同一接口有时是 `{}`，有时是 `[]`，有时是 `null`。
- 成功 HTTP status 必须按接口契约固定：创建资源用 201，其余读取、登录、更新、修改操作默认用 200。

成功示例：

```json
{
  "code": 0,
  "data": {
    "id": 1,
    "username": "alice",
    "email": "alice@example.com",
    "avatarUrl": null,
    "role": "user"
  },
  "message": "User registered successfully"
}
```

失败示例：

```json
{
  "code": 40002,
  "data": null,
  "message": "Email is already registered"
}
```

错误码范围：

| 范围 | 含义 | 示例 |
|---|---|---|
| `40000-40099` | 输入校验或请求格式错误 | 邮箱格式错误、密码太短 |
| `40100-40199` | 未登录或 token 无效 | 缺少 token、token 过期 |
| `40300-40399` | 权限不足 | 普通用户访问管理员接口 |
| `40400-40499` | 资源不存在 | 用户不存在、游戏不存在 |
| `40900-40999` | 冲突 | 邮箱已注册、用户名已占用 |
| `50000-50099` | 服务端错误 | 数据库错误、未知错误 |

---

## 9. Phase 2 用户系统字段映射表

### 9.1 User 字段

用户相关字段必须按下表统一。未在表中的字段不要擅自添加。

| 概念 | 数据库字段 | Rust 字段 | API JSON 字段 | 前端变量 | 说明 |
|---|---|---|---|---|---|
| 用户 ID | `id` | `id` | `id` | `currentUser.id` | 主键 |
| 用户名 | `username` | `username` | `username` | `username` | 唯一，注册后可否修改需单独约定 |
| 邮箱 | `email` | `email` | `email` | `email` | 唯一，后端保存前统一小写 |
| 密码哈希 | `password_hash` | `password_hash` | 不返回 | 不保存 | 只存在后端和数据库 |
| 头像 URL | `avatar_url` | `avatar_url` | `avatarUrl` | `avatarUrl` | 可为 `null` |
| 角色 | `role` | `role` | `role` | `role` | 初始值 `user`，管理员为 `admin` |
| 创建时间 | `created_at` | `created_at` | `createdAt` | `createdAt` | ISO 8601 字符串 |
| 更新时间 | `updated_at` | `updated_at` | `updatedAt` | `updatedAt` | ISO 8601 字符串 |

### 9.2 Auth 字段

| 概念 | Rust 字段 | API JSON 字段 | 前端变量 | 说明 |
|---|---|---|---|---|
| 登录邮箱 | `email` | `email` | `email` | 用户输入 |
| 登录密码 | `password` | `password` | `password` | 只在请求体中出现 |
| JWT | `token` | `token` | `authToken` | 不能写入 README 或日志 |
| token 类型 | `token_type` | `tokenType` | `tokenType` | 固定 `Bearer` |
| 过期秒数 | `expires_in` | `expiresIn` | `expiresIn` | 数字，单位秒 |

---

## 10. Phase 2 API 契约草案

本节是 Phase 2 开发时的默认契约。正式实现前，角色 A/B/C 必须确认并在必要时更新本节。

### 10.1 用户注册

| 项目 | 内容 |
|---|---|
| Method | `POST` |
| Path | `/api/auth/register` |
| Auth | 不需要 |
| Request DTO | `RegisterRequest` |
| Response DTO | `UserResponse` |
| Success HTTP | `201 Created` |
| 数据表 | `users` |
| 前端页面 | `RegisterView.vue` |

请求：

```json
{
  "username": "alice",
  "email": "alice@example.com",
  "password": "password123"
}
```

成功响应：

```json
{
  "code": 0,
  "data": {
    "id": 1,
    "username": "alice",
    "email": "alice@example.com",
    "avatarUrl": null,
    "role": "user",
    "createdAt": "2026-01-01T00:00:00Z",
    "updatedAt": "2026-01-01T00:00:00Z"
  },
  "message": "User registered successfully"
}
```

必须覆盖的失败场景：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 用户名为空或太短 | 400 | `40001` | `null` | `Username is invalid` |
| 邮箱格式错误 | 400 | `40002` | `null` | `Email is invalid` |
| 密码太短 | 400 | `40003` | `null` | `Password is too short` |
| 用户名已存在 | 409 | `40901` | `null` | `Username is already taken` |
| 邮箱已存在 | 409 | `40902` | `null` | `Email is already registered` |

### 10.2 用户登录

| 项目 | 内容 |
|---|---|
| Method | `POST` |
| Path | `/api/auth/login` |
| Auth | 不需要 |
| Request DTO | `LoginRequest` |
| Response DTO | `AuthTokenResponse` |
| Success HTTP | `200 OK` |
| 数据表 | `users` |
| 前端页面 | `LoginView.vue` |

请求：

```json
{
  "email": "alice@example.com",
  "password": "password123"
}
```

成功响应：

```json
{
  "code": 0,
  "data": {
    "token": "jwt-token-string",
    "tokenType": "Bearer",
    "expiresIn": 604800,
    "user": {
      "id": 1,
      "username": "alice",
      "email": "alice@example.com",
      "avatarUrl": null,
      "role": "user",
      "createdAt": "2026-01-01T00:00:00Z",
      "updatedAt": "2026-01-01T00:00:00Z"
    }
  },
  "message": "Login successful"
}
```

必须覆盖的失败场景：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 邮箱格式错误 | 400 | `40002` | `null` | `Email is invalid` |
| 密码为空 | 400 | `40004` | `null` | `Password is required` |
| 邮箱或密码错误 | 401 | `40101` | `null` | `Invalid email or password` |

### 10.3 获取当前用户

| 项目 | 内容 |
|---|---|
| Method | `GET` |
| Path | `/api/users/me` |
| Auth | 需要 `Authorization: Bearer <token>` |
| Request DTO | 无 |
| Response DTO | `UserResponse` |
| Success HTTP | `200 OK` |
| 数据表 | `users` |
| 前端页面 | `ProfileView.vue` |

成功响应：

```json
{
  "code": 0,
  "data": {
    "id": 1,
    "username": "alice",
    "email": "alice@example.com",
    "avatarUrl": null,
    "role": "user",
    "createdAt": "2026-01-01T00:00:00Z",
    "updatedAt": "2026-01-01T00:00:00Z"
  },
  "message": "Current user loaded"
}
```

失败场景：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 缺少 token | 401 | `40102` | `null` | `Authentication is required` |
| token 无效或过期 | 401 | `40103` | `null` | `Token is invalid or expired` |
| 用户不存在 | 404 | `40401` | `null` | `User not found` |

### 10.4 更新个人资料

| 项目 | 内容 |
|---|---|
| Method | `PATCH` |
| Path | `/api/users/me` |
| Auth | 需要 |
| Request DTO | `UpdateUserProfileRequest` |
| Response DTO | `UserResponse` |
| Success HTTP | `200 OK` |
| 数据表 | `users` |
| 前端页面 | `ProfileView.vue` |

请求：

```json
{
  "username": "alice_new"
}
```

成功响应：

```json
{
  "code": 0,
  "data": {
    "id": 1,
    "username": "alice_new",
    "email": "alice@example.com",
    "avatarUrl": null,
    "role": "user",
    "createdAt": "2026-01-01T00:00:00Z",
    "updatedAt": "2026-01-01T00:00:00Z"
  },
  "message": "Profile updated successfully"
}
```

规则：

- Phase 2 默认只允许改 `username`。
- 邮箱是否允许修改必须另开任务，不能顺手做。
- `avatarUrl` 只能由 `POST /api/users/me/avatar` 上传成功后由后端更新，不能通过本接口直接提交 URL。
- 请求体中未出现的字段不更新。

失败场景：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 缺少 token | 401 | `40102` | `null` | `Authentication is required` |
| token 无效或过期 | 401 | `40103` | `null` | `Token is invalid or expired` |
| 用户名格式错误 | 400 | `40001` | `null` | `Username is invalid` |
| 请求体包含 `avatarUrl` | 400 | `40005` | `null` | `Avatar URL cannot be updated directly` |
| 用户名已存在 | 409 | `40901` | `null` | `Username is already taken` |

### 10.5 修改密码

| 项目 | 内容 |
|---|---|
| Method | `PATCH` |
| Path | `/api/users/me/password` |
| Auth | 需要 |
| Request DTO | `ChangePasswordRequest` |
| Response DTO | 空对象 `{}` |
| Success HTTP | `200 OK` |
| 数据表 | `users` |
| 前端页面 | `ProfileView.vue` |

请求：

```json
{
  "currentPassword": "old-password",
  "newPassword": "new-password123"
}
```

成功响应：

```json
{
  "code": 0,
  "data": {},
  "message": "Password changed successfully"
}
```

失败场景：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 缺少 token | 401 | `40102` | `null` | `Authentication is required` |
| token 无效或过期 | 401 | `40103` | `null` | `Token is invalid or expired` |
| 当前密码为空 | 400 | `40004` | `null` | `Password is required` |
| 新密码太短 | 400 | `40003` | `null` | `Password is too short` |
| 当前密码错误 | 401 | `40104` | `null` | `Current password is incorrect` |

### 10.6 头像上传

| 项目 | 内容 |
|---|---|
| Method | `POST` |
| Path | `/api/users/me/avatar` |
| Auth | 需要 |
| Request DTO | `multipart/form-data`，字段名 `avatar` |
| Response DTO | `{ "avatarUrl": string }` |
| Success HTTP | `200 OK` |
| 数据表 | `users` |
| 前端页面 | `ProfileView.vue` |

存储策略：

- Phase 2 使用本地开发存储，文件保存到 `UPLOAD_DIR/avatars`；默认 `UPLOAD_DIR=uploads`，相对 `server/` 解析为 `server/uploads/avatars/`。
- API 返回相对公开 URL，默认 `UPLOAD_PUBLIC_BASE_URL=/uploads`，头像成功响应形如 `/uploads/avatars/user-{id}-{timestamp}.{ext}`。
- 后端提供 `GET /uploads/avatars/{file_name}` 静态读取头像文件；该静态路由成功时返回图片 bytes，不包裹 API envelope。
- 最大大小由 `MAX_AVATAR_SIZE_BYTES` 控制，默认 `2097152`（2 MiB）。
- 允许 MIME 与文件签名：`image/png`、`image/jpeg`、`image/webp`。
- 本地上传内容必须由 `.gitignore` 排除，不能提交真实用户上传文件。

成功响应：

```json
{
  "code": 0,
  "data": {
    "avatarUrl": "/uploads/avatars/user-1-1780887306586.png"
  },
  "message": "Avatar uploaded successfully"
}
```

失败场景：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 缺少 token | 401 | `40102` | `null` | `Authentication is required` |
| token 无效或过期 | 401 | `40103` | `null` | `Token is invalid or expired` |
| 未提交 `avatar` 字段 | 400 | `40006` | `null` | `Avatar file is required` |
| 文件类型不允许或签名不匹配 | 400 | `40007` | `null` | `Avatar file type is not allowed` |
| 文件超过大小限制 | 400 | `40008` | `null` | `Avatar file is too large` |
| 用户不存在 | 404 | `40401` | `null` | `User not found` |
| 静态头像文件不存在 | 404 | `40402` | `null` | `Uploaded file not found` |

当前前端 `ProfileView.vue` 仍只展示头像占位和“待接入上传接口”按钮，不调用本接口；Role B 接入上传交互前，不得把前端头像上传标记完成。

---

## 11. Phase 3 以后字段预留规范

后续游戏、分类、标签、评论、点赞、收藏、下载链接、截图字段必须沿用同一映射规则。

### 11.1 Game 字段

| 概念 | 数据库字段 | Rust 字段 | API JSON 字段 | 前端变量 |
|---|---|---|---|---|
| 游戏 ID | `id` | `id` | `id` | `game.id` |
| 标题 | `title` | `title` | `title` | `title` |
| 开发商 | `developer` | `developer` | `developer` | `developer` |
| 发行商 | `publisher` | `publisher` | `publisher` | `publisher` |
| 发售日期 | `release_date` | `release_date` | `releaseDate` | `releaseDate` |
| 简介 | `description` | `description` | `description` | `description` |
| 封面 URL | `cover_url` | `cover_url` | `coverUrl` | `coverUrl` |
| 分类 ID | `category_id` | `category_id` | `categoryId` | `categoryId` |
| 搜索文本 | `search_text` | `search_text` | 不直接返回 | 不直接使用 |
| 点赞数 | `likes_count` | `likes_count` | `likesCount` | `likesCount` |
| 收藏数 | `favorites_count` | `favorites_count` | `favoritesCount` | `favoritesCount` |

### 11.2 Category 和 Tag 字段

| 概念 | 数据库字段 | Rust 字段 | API JSON 字段 | 前端变量 |
|---|---|---|---|---|
| ID | `id` | `id` | `id` | `id` |
| 名称 | `name` | `name` | `name` | `name` |
| Slug | `slug` | `slug` | `slug` | `slug` |

### 11.2.1 Phase 3 前端交接状态（2026-06-10）

Role B 已完成 Phase 3 游戏浏览前端实现，当前状态是“前端可预览，真实后端联调待完成”。

已完成前端文件：

```txt
client/src/api/games.js
client/src/views/game/GameListView.vue
client/src/views/game/GameDetailView.vue
client/src/components/game/GameCard.vue
client/src/components/game/GameFilter.vue
client/src/components/game/ScreenshotCarousel.vue
client/src/components/common/Pagination.vue
client/src/components/common/BaseLoading.vue
client/src/components/common/EmptyState.vue
client/src/router/index.js
client/src/style.css
```

前端 API client 已按现有命名风格使用 `games.js`，并基于 `client/src/api/http.js`；没有创建 `request.js`。页面内部统一使用 camelCase，`games.js` 中负责把后端可能返回的 `cover_url`、`release_date`、`likes_count`、`favorites_count`、`sort_order` 转换为 `coverUrl`、`releaseDate`、`likesCount`、`favoritesCount`、`sortOrder`。

当前前端函数：

```txt
getGames(params)
getGameDetail(id)
getCategories()
getTags()
```

前端暂未默认新增 `getGameScreenshots(id)`；当前约定优先从 `GET /api/games/:id` 的 `screenshots` 字段读取截图。若后端决定提供 `GET /api/games/:id/screenshots`，需要先更新契约，再由 Role B 补 API client。

mock fallback 说明：

- 当前 mock fallback 只用于 UI 预览和开发兜底。
- 不允许把 mock fallback 标记为真实接口联调完成。
- 当后端接口可用后，需要重新验证 `/games`、筛选 query、分页、`/games/:id`、图片 URL 和截图展示。

Phase 3 前端验证结果：

```bash
npm --prefix client run build
npm run lint
```

以上命令均已通过。浏览器已验证 `/games`、`/games?page=1&categoryId=1&tagId=1`、`/games/1?page=1&categoryId=1&tagId=1` 可渲染，详情页返回列表时会保留 query。

Phase 3 后端/数据库待确认事项：

- `games` 表已由 `server/migrations/20260612000000_create_games.sql` 定义。
- `categories` 表已由 `server/migrations/20260612000000_create_games.sql` 定义。
- `tags` 表已由 `server/migrations/20260612000000_create_games.sql` 定义。
- `game_tags` 表已由 `server/migrations/20260612000000_create_games.sql` 定义。
- `screenshots` 表已由 `server/migrations/20260612000000_create_games.sql` 定义。
- seed 假数据已由 `server/seeds/dev_phase3_games.sql` 提供。
- `GET /api/games` 已实现，返回 `{ list, total, page, pageSize }`。
- `GET /api/games/:id` 已实现，详情内嵌 `category`、`tags`、`screenshots`。
- `GET /api/categories` 已实现。
- `GET /api/tags` 已实现。
- 分页参数确认为 `page` / `pageSize`。
- 筛选参数确认为 `categoryId` / `tagId`。
- 图片 URL 当前按数据库中存储的相对/原样字符串返回；开发 seed 暂用空字符串占位。
- `screenshots` 包含在详情接口中，不提供独立接口。
- `category` / `tags` 字段格式为 `{ id, name, slug }`，与前端一致。
- 仍需在可用 PostgreSQL 环境中跑真实 `/games` 前后端联调。

### 11.3 Comment 字段

| 概念 | 数据库字段 | Rust 字段 | API JSON 字段 | 前端变量 |
|---|---|---|---|---|
| 评论 ID | `id` | `id` | `id` | `comment.id` |
| 用户 ID | `user_id` | `user_id` | `userId` | `userId` |
| 用户名 | `users.username` | `username` | `username` | `username` |
| 用户头像 URL | `users.avatar_url` | `avatar_url` | `avatarUrl` | `avatarUrl` |
| 游戏 ID | `game_id` | `game_id` | `gameId` | `gameId` |
| 内容 | `content` | `content` | `content` | `content` |
| 父评论 ID | `parent_id` | `parent_id` | `parentId` | `parentId` |
| 创建时间 | `created_at` | `created_at` | `createdAt` | `createdAt` |

### 11.4 Like 和 Favorite 字段

| 概念 | 数据库字段 | Rust 字段 | API JSON 字段 | 前端变量 |
|---|---|---|---|---|
| 用户 ID | `user_id` | `user_id` | 不单独返回 | `currentUser.id` |
| 游戏 ID | `game_id` | `game_id` | 不单独返回 | `game.id` |
| 点赞状态 | `likes` 是否存在行 | `liked` | `liked` | `liked` |
| 点赞数 | `games.likes_count` | `likes_count` | `likesCount` | `likesCount` |
| 收藏状态 | `favorites` 是否存在行 | `favorited` | `favorited` | `favorited` |
| 收藏数 | `games.favorites_count` | `favorites_count` | `favoritesCount` | `favoritesCount` |
| 创建时间 | `created_at` | `created_at` | 列表排序使用，不单独返回 | 不直接使用 |

### 11.5 Phase 4 互动后端 API 契约摘要

所有接口继续使用统一 envelope `{ code, data, message }`，成功业务码为 `0`，认证接口使用 `Authorization: Bearer <token>`。后端是评论内容和权限校验的最终可信来源；前端可以做体验校验，但不能替代后端。

| Method | Path | Auth | Request DTO | Success HTTP | Success data |
|---|---|---|---|---|---|
| `GET` | `/api/games/{gameId}/comments?page=&pageSize=` | 不需要 | query `page` / `pageSize` | 200 | `{ list,total,page,pageSize }`，评论项含 `id,userId,username,avatarUrl,gameId,content,parentId,createdAt` |
| `POST` | `/api/games/{gameId}/comments` | 需要 | `{ content, parentId? }` | 201 | `CommentResponse`，message 固定 `Comment created successfully` |
| `DELETE` | `/api/comments/{id}` | 需要 | 无 | 200 | `{}`，message 固定 `Comment deleted successfully` |
| `POST` | `/api/games/{gameId}/like` | 需要 | 无 | 200 | `{ liked:true, likesCount }` |
| `DELETE` | `/api/games/{gameId}/like` | 需要 | 无 | 200 | `{ liked:false, likesCount }` |
| `POST` | `/api/games/{gameId}/favorite` | 需要 | 无 | 200 | `{ favorited:true, favoritesCount }` |
| `DELETE` | `/api/games/{gameId}/favorite` | 需要 | 无 | 200 | `{ favorited:false, favoritesCount }` |
| `GET` | `/api/users/me/favorites?page=&pageSize=` | 需要 | query `page` / `pageSize` | 200 | 现有 `GameListResponse`：`{ list,total,page,pageSize }`；列表项 `screenshots` 可为空数组 |

Phase 4 后端错误码：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 评论内容 trim 后为空 | 400 | `40009` | `null` | `Comment content is required` |
| 评论内容超过 1000 字符 | 400 | `40010` | `null` | `Comment content is too long` |
| 缺少 token | 401 | `40102` | `null` | `Authentication is required` |
| token 无效或过期 | 401 | `40103` | `null` | `Token is invalid or expired` |
| 删除他人评论且非 admin | 403 | `40301` | `null` | `Permission denied` |
| 游戏不存在 | 404 | `40403` | `null` | `Game not found` |
| 评论或跨游戏父评论不存在 | 404 | `40404` | `null` | `Comment not found` |

Phase 4 数据库契约：`comments` / `likes` / `favorites` 由 `server/migrations/20260613000000_create_interactions.sql` 创建。`comments.user_id`、`comments.game_id`、`likes.user_id`、`likes.game_id`、`favorites.user_id`、`favorites.game_id` 均级联删除；`comments.parent_id` 自引用级联删除；`likes` 和 `favorites` 以 `(user_id, game_id)` 为主键，写入接口必须幂等并刷新 `games.likes_count` / `games.favorites_count`。

### 11.6 Phase 5 管理后台与资源后端契约摘要

管理员接口继续使用统一 envelope `{ code, data, message }`。所有 `/api/admin/...` 路由需要 `Authorization: Bearer <token>`，缺失/无效 token 沿用现有 `40102` / `40103`；已登录但 `users.role != 'admin'` 返回 HTTP 403、`code=40301`、`message="Permission denied"`。

| Method | Path | Auth | Request DTO | Success HTTP | Success data |
|---|---|---|---|---|---|
| `POST` | `/api/admin/uploads/images` | admin | `multipart/form-data` 字段 `image` | 200 | `{ imageUrl }`，URL 形如 `/uploads/images/image-{adminId}-{timestamp}.png` |
| `GET` | `/api/admin/games?page=&pageSize=&categoryId=&tagId=` | admin | query 同公开游戏列表 | 200 | 现有 `GameListResponse` |
| `POST` | `/api/admin/games` | admin | `CreateGameRequest` | 201 | `GameResponse` |
| `PUT` | `/api/admin/games/{gameId}` | admin | `UpdateGameRequest`（同创建字段） | 200 | `GameResponse` |
| `DELETE` | `/api/admin/games/{gameId}` | admin | 无 | 200 | `{}` |
| `GET` | `/api/admin/games/{gameId}/download-links` | admin | 无 | 200 | `DownloadLinkResponse[]` |
| `POST` | `/api/admin/games/{gameId}/download-links` | admin | `DownloadLinkRequest` | 201 | `DownloadLinkResponse` |
| `PUT` | `/api/admin/games/{gameId}/download-links/{id}` | admin | `DownloadLinkRequest` | 200 | `DownloadLinkResponse` |
| `DELETE` | `/api/admin/games/{gameId}/download-links/{id}` | admin | 无 | 200 | `{}` |
| `GET` | `/api/games/{gameId}/download-links` | 不需要 | 无 | 200 | `DownloadLinkResponse[]`，供前台下载区域读取 |

`CreateGameRequest` / `UpdateGameRequest` 字段：`title`, `developer`, `publisher`, `releaseDate?` (`YYYY-MM-DD` 或 `null`), `description`, `coverUrl?`, `categoryId`, `tagIds`, `screenshots`。`screenshots` 项为 `{ url, sortOrder? }`；后端创建/更新时在事务中替换 `game_tags` 和 `screenshots`，并更新 `games.search_text`。

`DownloadLinkResponse` 字段：`id`, `gameId`, `platform`, `url`, `extractCode`, `password`, `fileSize`, `createdAt`, `updatedAt`。`extractCode` / `password` 是产品契约内的下载提取信息，不得记录真实生产链接或真实密码到 README/JOURNALIST 日志。

Phase 5 后端错误码：

| 场景 | HTTP | code | data | message |
|---|---|---|---|---|
| 管理员图片未提交 `image` 字段 | 400 | `40011` | `null` | `Image file is required` |
| 管理员图片 MIME 或签名不允许 | 400 | `40012` | `null` | `Image file type is not allowed` |
| 管理员图片超过大小限制 | 400 | `40013` | `null` | `Image file is too large` |
| 游戏创建/更新字段无效 | 400 | `40014` | `null` | `Game field is invalid` |
| 下载链接字段无效 | 400 | `40015` | `null` | `Download link is invalid` |
| 非管理员访问管理员接口 | 403 | `40301` | `null` | `Permission denied` |
| 游戏不存在 | 404 | `40403` | `null` | `Game not found` |
| 分类不存在 | 404 | `40405` | `null` | `Category not found` |
| 标签不存在 | 404 | `40406` | `null` | `Tag not found` |
| 下载链接不存在 | 404 | `40407` | `null` | `Download link not found` |

Phase 5 数据库契约：`download_links` 由 `server/migrations/20260614000000_create_download_links.sql` 创建，字段为 `id`, `game_id`, `platform`, `url`, `extract_code`, `password`, `file_size`, `created_at`, `updated_at`；`game_id` 引用 `games(id) ON DELETE CASCADE`，删除游戏会级联清理下载链接。Phase 5 不重复创建 Phase 3 已有的 `games` / `categories` / `tags` / `game_tags` / `screenshots` 表。

---

## 12. 环境变量契约

已有环境变量：

| 变量 | 位置 | 用途 | 默认值 |
|---|---|---|---|
| `PORT` | root/server `.env.example` | 后端端口 | `3000` |
| `HOST` | root/server `.env.example` | 后端监听地址 | `127.0.0.1` |
| `RUST_LOG` | root/server `.env.example` | Rust 日志级别 | `info` |
| `POSTGRES_HOST` | root/server `.env.example` | PostgreSQL host | `localhost` |
| `POSTGRES_PORT` | root/server `.env.example` | PostgreSQL port | `5432` |
| `POSTGRES_DB` | root/server `.env.example` | 数据库名 | `nonewhite_site` |
| `POSTGRES_USER` | root/server `.env.example` | 数据库用户 | `nonewhite_user` |
| `POSTGRES_PASSWORD` | root/server `.env.example` | 数据库密码 | `nonewhite_password` |
| `DATABASE_URL` | root/server `.env.example` | 后端连接字符串 | `postgres://...` |

Phase 2 已新增：

| 变量 | 位置 | 用途 | 规则/默认值 |
|---|---|---|---|
| `JWT_SECRET` | `server/.env.example` 和根 `.env.example` | JWT 签名密钥 | 示例值只能是开发占位，不能提交真实 secret |
| `JWT_EXPIRES_IN_SECONDS` | `server/.env.example` 和根 `.env.example` | JWT 过期时间 | 默认 `604800`，数字，单位秒 |
| `UPLOAD_DIR` | `server/.env.example` 和根 `.env.example` | 上传文件目录 | 默认 `uploads`，相对 `server/`；本地上传目录必须被 Git 忽略 |
| `UPLOAD_PUBLIC_BASE_URL` | `server/.env.example` 和根 `.env.example` | 上传文件公开 URL 前缀 | 默认 `/uploads` |
| `MAX_AVATAR_SIZE_BYTES` | `server/.env.example` 和根 `.env.example` | 头像最大大小 | 默认 `2097152`，数字，单位 byte |
| `MAX_IMAGE_SIZE_BYTES` | `server/.env.example` 和根 `.env.example` | 管理员通用图片最大大小 | 默认 `5242880`，数字，单位 byte |

前端环境变量规则：

- Vite 暴露给前端的变量必须以 `VITE_` 开头。
- 默认不要新增 `VITE_API_BASE_URL`，开发环境使用相对路径 `/api`。
- 如果部署阶段需要前端 API base URL，必须在 Phase 6 单独设计。

新增环境变量时必须同步更新：

1. 根目录 `.env.example`
2. `server/.env.example` 或前端 env 示例文件
3. README 本地启动说明
4. 本文档环境变量表

后端环境变量加载规则：

- Phase 2 以后以后端 `server/.env` 为主要后端配置来源。
- 根目录 `.env` 主要供 `docker-compose.yml` 和项目级默认值使用。
- 如果两个文件存在同名变量，后端实现必须明确优先级，推荐 `server/.env` 优先于根目录 `.env`。
- 不要在 `config.rs`、`db.rs`、service 中重复调用 dotenv 加载；环境加载应集中在启动阶段完成。
- `DATABASE_URL`、`JWT_SECRET`、`JWT_EXPIRES_IN_SECONDS` 的最终读取结果必须能在启动错误中定位缺失变量名，但不能打印 secret 值。

---

## 13. 数据库 schema 和 migration 规则

当前项目还没有 migration 工具。角色 C 和 A 必须先选择一种方式，再开始建表。

允许的方向：

- 如果使用 SQL 文件：统一放在 `server/migrations/`。
- 如果使用 Rust migration 工具：必须在 `server/Cargo.toml` 中记录依赖，并在 README 写明命令。
- 不允许一部分表用 SQL 文件、一部分表用另一套工具。

migration 命名建议：

```txt
YYYYMMDDHHMMSS_create_users.sql
YYYYMMDDHHMMSS_create_games.sql
```

用户表建议字段，正式实现前仍需由 A/C 确认：

```sql
id
username
email
password_hash
avatar_url
role
created_at
updated_at
```

约束建议：

- `username` 唯一。
- `email` 唯一。
- `role` 默认 `user`。
- `avatar_url` 可为空。
- `created_at` 和 `updated_at` 必须有默认时间或由后端写入。

不能做：

- 不能把本地 Docker volume、真实 `.env`、真实数据库 dump 提交到 Git。
- 不能在没有 migration 的情况下把 README 写成“建表完成”。
- 不能只改 Rust model 不落数据库 schema。

### 13.1 数据库实现框架

Phase 2 开始时，角色 C 必须先把数据库文件结构固定下来。推荐最小结构：

```txt
server/
  migrations/
    20260605000000_create_users.sql
  seeds/
    dev_users.sql
  docs/
    database.md
```

如果不创建 `server/docs/database.md`，也可以把数据库说明写入 `docs/database/phase2-users.md`，但只能二选一。不要同时维护两份数据库说明。

每个 migration 文件必须包含：

1. 这个 migration 创建或修改什么表。
2. 所有字段、类型、默认值、是否允许 null。
3. 主键、唯一约束、外键、索引。
4. 和本文档字段映射表是否一致。
5. 如果当前 migration 工具支持 rollback，必须提供 rollback 文件或 rollback 说明。

### 13.2 `users` 表推荐实现草案

如果项目没有另行决定，Phase 2 默认按下面 schema 开始：

```sql
CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  username VARCHAR(32) NOT NULL UNIQUE,
  email VARCHAR(255) NOT NULL UNIQUE,
  password_hash TEXT NOT NULL,
  avatar_url TEXT,
  role VARCHAR(32) NOT NULL DEFAULT 'user',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX idx_users_email ON users (email);
CREATE INDEX idx_users_username ON users (username);
```

字段解释：

| 字段 | 类型建议 | 必填 | 默认值 | 说明 |
|---|---|---|---|---|
| `id` | `BIGSERIAL` | 是 | 自动生成 | 主键，API 直接返回数字 |
| `username` | `VARCHAR(32)` | 是 | 无 | 3-32 字符，唯一 |
| `email` | `VARCHAR(255)` | 是 | 无 | 保存小写值，唯一 |
| `password_hash` | `TEXT` | 是 | 无 | bcrypt hash，不返回给前端 |
| `avatar_url` | `TEXT` | 否 | `NULL` | 头像地址，API 映射为 `avatarUrl` |
| `role` | `VARCHAR(32)` | 是 | `user` | 允许值先限定为 `user` / `admin` |
| `created_at` | `TIMESTAMPTZ` | 是 | `NOW()` | 创建时间 |
| `updated_at` | `TIMESTAMPTZ` | 是 | `NOW()` | 更新时间 |

实现前必须确认的问题：

- 是否用 `BIGSERIAL` 还是 UUID。默认用 `BIGSERIAL`，因为 README 初版表设计只写 `id`，当前项目未引入 UUID 依赖。
- 是否在数据库层增加 `role` check constraint。默认可以先不加，后端校验即可；如果加，必须写入文档。
- `updated_at` 是否由数据库 trigger 自动更新。默认 Phase 2 可由后端更新，后续再优化。

### 13.3 数据访问层约定

角色 A 选择数据库访问库前，必须在 `server/Cargo.toml` 的变更说明中写清楚原因。推荐优先考虑 `sqlx`，因为它适合 Rust + PostgreSQL + async axum 项目。

如果选择 `sqlx`，推荐后端目录：

```txt
server/src/db.rs
server/src/repositories/user_repository.rs
server/src/models/user.rs
```

职责：

| 文件 | 职责 |
|---|---|
| `db.rs` | 创建 PostgreSQL 连接池，读取 `DATABASE_URL` |
| `models/user.rs` | 数据库行结构，例如 `UserRow` |
| `repositories/user_repository.rs` | 所有 `users` 表 SQL 操作 |

repository 函数命名：

```txt
create_user
find_user_by_id
find_user_by_email
find_user_by_username
update_user_profile
update_user_password_hash
update_user_avatar_url
```

repository 只返回数据库结果或领域错误，不直接返回 HTTP response。

### 13.4 seed 数据规则

seed 只用于本地开发和联调，不能包含真实密码。推荐：

```txt
server/seeds/dev_users.sql
```

seed 规则：

- 文件名必须带 `dev_`，明确是开发数据。
- 密码必须使用公开测试密码，例如 `password123`。
- seed 里如果包含 hash，必须写明明文测试密码是什么。
- README 只能写“开发 seed”，不能写成生产初始化数据。

---

## 14. 后端实现规范

当前后端结构：

```txt
server/src/main.rs
server/src/config.rs
server/src/response.rs
server/src/routes/mod.rs
server/src/routes/test.rs
```

新增模块时优先保持清晰：

```txt
server/src/routes/auth.rs
server/src/routes/users.rs
server/src/dto/auth.rs
server/src/dto/users.rs
server/src/models/user.rs
server/src/services/auth_service.rs
server/src/middleware/auth.rs
```

实现规则：

- handler 负责 HTTP 提取和返回。
- service 负责业务逻辑。
- repository 或数据库访问层负责 SQL/DB 操作。
- DTO 负责请求和响应结构。
- error 模块负责把错误转换为统一 API envelope。
- 不要在 route handler 里堆全部逻辑。
- 不要把数据库字段原样泄漏给前端。

认证规则：

- 注册时保存 `password_hash`，永远不保存明文密码。
- 登录失败 message 不区分邮箱不存在还是密码错误，统一 `Invalid email or password`。
- 需要登录的接口必须读取 `Authorization: Bearer <token>`。
- token 解析失败必须返回 401 envelope。

### 14.1 后端目标目录框架

Phase 2 完成后，后端推荐形成下面的最小框架。不要一次性创建所有空文件，按任务逐步创建；但最终结构应靠近这个框架。

```txt
server/src/
  main.rs
  config.rs
  db.rs
  error.rs
  response.rs
  routes/
    mod.rs
    test.rs
    auth.rs
    users.rs
  dto/
    mod.rs
    auth.rs
    users.rs
  models/
    mod.rs
    user.rs
  repositories/
    mod.rs
    user_repository.rs
  services/
    mod.rs
    auth_service.rs
    user_service.rs
  middleware/
    mod.rs
    auth.rs
```

模块职责必须固定：

| 层 | 允许做什么 | 禁止做什么 |
|---|---|---|
| `routes` | 定义 URL、提取 JSON/path/header、调用 service、返回 `ApiResponse` | 直接写 SQL、直接 hash 密码、堆业务流程 |
| `dto` | 定义 request/response struct、字段序列化规则 | 连接数据库、写业务逻辑 |
| `models` | 表示数据库行或领域实体 | 返回 HTTP response |
| `repositories` | 读写数据库 | 读取 HTTP header、生成 JWT |
| `services` | 业务流程、校验、hash、token、调用 repository | 直接依赖前端字段名之外的临时格式 |
| `middleware` | 认证提取、token 校验、注入当前用户 | 查询不相关业务数据 |
| `error.rs` | 统一错误类型和错误码映射 | 在各 handler 中分散写错误 JSON |

### 14.2 后端依赖框架

Phase 2 后端可能需要新增依赖。新增前必须只选择当前任务需要的依赖，不要一次性堆依赖。

推荐方向：

| 能力 | 推荐依赖方向 | 用途 |
|---|---|---|
| PostgreSQL async | `sqlx` with postgres/runtime-tokio | 连接池和 SQL 查询 |
| 密码哈希 | `bcrypt` | README 已约定 JWT + bcrypt |
| JWT | `jsonwebtoken` | 签发和校验 token |
| 时间 | `chrono` 或 `time` | `createdAt`、`updatedAt`、token 过期 |
| HTTP header | axum/http 已包含 | 读取 Authorization |
| multipart | axum multipart feature 或独立方案 | Phase 2 头像上传时再引入 |

新增依赖时必须同步做三件事：

1. 修改 `server/Cargo.toml`。
2. 运行 `cargo check --manifest-path server/Cargo.toml`。
3. 在交接说明中写明新增依赖和用途。

### 14.3 `config.rs` 目标职责

`config.rs` 不应该只返回 server address。Phase 2 后建议拆成配置结构：

```rust
pub struct ServerConfig {
    pub host: String,
    pub port: String,
}

pub struct DatabaseConfig {
    pub database_url: String,
}

pub struct AuthConfig {
    pub jwt_secret: String,
    pub jwt_expires_in_seconds: u64,
}

pub struct AppConfig {
    pub server: ServerConfig,
    pub database: DatabaseConfig,
    pub auth: AuthConfig,
}
```

规则：

- `HOST`、`PORT` 可以有默认值。
- `DATABASE_URL`、`JWT_SECRET` 在需要真实连接/认证时不能静默缺失。
- 不要在多个文件里重复读取同一个环境变量。
- 不要把 secret 打印到日志。

### 14.4 `AppState` 框架

一旦后端需要数据库或认证配置，`main.rs` 应创建共享状态并注入 router。推荐概念结构：

```rust
#[derive(Clone)]
pub struct AppState {
    pub db: PgPool,
    pub auth: AuthConfig,
}
```

使用规则：

- handler 通过 `State<AppState>` 获取连接池和配置。
- 不要在每个请求里重新创建数据库连接池。
- 不要把 `AppState` 变成全局 mutable 单例。

### 14.5 DTO 框架

Phase 2 DTO 推荐：

```txt
server/src/dto/auth.rs
  RegisterRequest
  LoginRequest
  AuthTokenResponse

server/src/dto/users.rs
  UserResponse
  UpdateUserProfileRequest
  ChangePasswordRequest
  AvatarUploadResponse
```

字段必须和 API 契约一致。Rust 字段如果使用 `snake_case`，对外 JSON 必须序列化成 `camelCase`。可以使用 `serde(rename_all = "camelCase")`，但不能让前端收到 `avatar_url`。

示例：

```rust
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
pub struct UserResponse {
    pub id: i64,
    pub username: String,
    pub email: String,
    pub avatar_url: Option<String>,
    pub role: String,
    pub created_at: String,
    pub updated_at: String,
}
```

禁止：

- 在 response DTO 中出现 `password`。
- 在 response DTO 中出现 `password_hash`。
- 为同一个接口创建多个相似 DTO，例如 `UserDto`、`UserInfo`、`UserProfile` 混用。

### 14.6 错误处理框架

必须集中定义应用错误，推荐：

```txt
server/src/error.rs
```

概念结构：

```rust
pub enum AppError {
    Validation { code: u16, message: String },
    Unauthorized { code: u16, message: String },
    Forbidden { code: u16, message: String },
    NotFound { code: u16, message: String },
    Conflict { code: u16, message: String },
    Internal { code: u16, message: String },
}
```

规则：

- handler 返回 `Result<Json<ApiResponse<T>>, AppError>` 或等价形式。
- `AppError` 统一转换为 `(StatusCode, Json<ApiResponse<Value>>)`。
- 数据库原始错误不要直接暴露到 `message`。
- 注册重复邮箱/用户名必须映射为 409。

### 14.7 Service 框架

`auth_service.rs` 推荐函数：

```txt
validate_register_request
validate_login_request
register_user
login_user
hash_password
verify_password
issue_token
verify_token
```

`user_service.rs` 推荐函数：

```txt
get_current_user
update_profile
change_password
update_avatar
```

service 规则：

- 校验逻辑优先放 service，便于测试。
- handler 不直接调用 `bcrypt` 或 JWT 库。
- service 返回 DTO 或领域对象，不返回 Vue 需要的临时结构。

### 14.8 后端测试框架

推荐测试位置：

```txt
server/tests/auth_api.rs
server/tests/users_api.rs
server/src/services/auth_service.rs  // 单元测试可放同文件 tests module
```

最小测试清单：

| 测试 ID | 位置 | 场景 | 断言 |
|---|---|---|---|
| `register_success_returns_user` | `server/tests/auth_api.rs` | 注册成功 | HTTP 201，`code=0`，无 `password_hash` |
| `register_rejects_invalid_email` | `server/tests/auth_api.rs` | 邮箱格式错误 | HTTP 400，`code=40002` |
| `register_rejects_duplicate_email` | `server/tests/auth_api.rs` | 重复邮箱 | HTTP 409，`code=40902` |
| `login_success_returns_token_and_user` | `server/tests/auth_api.rs` | 登录成功 | 返回 `token`、`tokenType=Bearer`、`user` |
| `login_rejects_wrong_password` | `server/tests/auth_api.rs` | 密码错误 | HTTP 401，`code=40101` |
| `me_requires_token` | `server/tests/users_api.rs` | 缺 token | HTTP 401，`code=40102` |
| `me_returns_current_user` | `server/tests/users_api.rs` | token 有效 | 返回当前用户 |

如果数据库测试暂时无法稳定运行，必须先写 service 层纯函数测试和手动 curl 场景，不能完全没有测试说明。

---

## 15. 前端实现规范

当前前端结构：

```txt
client/src/main.js
client/src/App.vue
client/src/router/index.js
client/src/api/test.js
client/src/views/HomeView.vue
client/src/views/TestApiView.vue
client/src/components/AppHeader.vue
client/src/components/AppFooter.vue
client/src/style.css
```

新增 auth 功能建议：

```txt
client/src/api/auth.js
client/src/api/users.js
client/src/views/LoginView.vue
client/src/views/RegisterView.vue
client/src/views/ProfileView.vue
client/src/components/auth/AuthForm.vue
client/src/stores/auth.js
```

API client 规则：

- 每个功能一个 API 文件，例如 `auth.js`、`users.js`。
- API client 只负责请求、解析 JSON、处理 HTTP 非 2xx。
- 页面组件负责展示 loading/error/success。
- 不要在多个页面重复写 `fetch('/api/...')`。

示例结构：

```js
export async function login(payload) {
  const response = await fetch('/api/auth/login', {
    method: 'POST',
    headers: { 'Content-Type': 'application/json' },
    body: JSON.stringify(payload),
  })

  const body = await response.json()

  if (!response.ok || body.code !== 0) {
    throw new Error(body.message || `请求失败：${response.status}`)
  }

  return body.data
}
```

路由规则：

- 新页面必须在 `client/src/router/index.js` 注册。
- `meta.label` 用于 Header 导航展示时，必须是中文短标签。
- 如果某页面不应该出现在主导航，需要先调整 Header 的路由过滤规则，不要删除 `meta.label` 造成不一致。

认证状态规则：

- 如果引入 store，统一命名为 `auth`。
- 当前用户变量使用 `currentUser`。
- token 变量使用 `authToken`。
- 退出登录至少要清除 `authToken` 和 `currentUser`。
- token 存储位置必须先约定，不能一个页面用 localStorage、另一个页面用 sessionStorage。

### 15.1 前端目标目录框架

Phase 2 完成后，前端推荐形成下面的最小框架：

```txt
client/src/
  api/
    http.js
    test.js
    auth.js
    users.js
  components/
    AppHeader.vue
    AppFooter.vue
    auth/
      AuthForm.vue
      AuthError.vue
    profile/
      ProfileForm.vue
      PasswordForm.vue
      AvatarUploader.vue
      FavoritesPlaceholder.vue
  router/
    index.js
  stores/
    auth.js
  views/
    HomeView.vue
    TestApiView.vue
    LoginView.vue
    RegisterView.vue
    ProfileView.vue
  utils/
    validation.js
  style.css
```

创建规则：

- `api/http.js` 先于 `auth.js` 和 `users.js` 创建，用于统一解析 envelope。
- `stores/auth.js` 只有在确定使用 Pinia 或轻量自写 store 时创建。
- 如果不用 Pinia，必须在本节记录替代方案，不要留下空 `stores/`。
- 页面先保证可路由、可 build，再逐步接入真实接口。

### 15.2 前端 HTTP client 框架

所有 API 文件必须复用统一请求函数。推荐 `client/src/api/http.js`：

```js
export async function requestJson(path, options = {}) {
  const response = await fetch(path, options)
  const body = await response.json()

  if (!response.ok || body.code !== 0) {
    throw new Error(body.message || `请求失败：${response.status}`)
  }

  return body.data
}
```

后续如果需要带 token，扩展同一个文件，不要在每个 API 文件重复写 header：

```js
export function createAuthHeaders(authToken) {
  return authToken ? { Authorization: `Bearer ${authToken}` } : {}
}
```

`auth.js` 推荐函数：

```txt
register(payload)
login(payload)
logoutLocal()
```

`users.js` 推荐函数：

```txt
fetchCurrentUser(authToken)
updateCurrentUser(payload, authToken)
changePassword(payload, authToken)
uploadAvatar(file, authToken)
```

命名规则：

- 请求 payload 字段必须和 API 契约一致：`username`、`email`、`password`、`currentPassword`、`newPassword`。
- 返回数据不做随意改名。后端返回 `avatarUrl`，前端也使用 `avatarUrl`。

### 15.3 认证状态框架

Phase 2 默认 token 存储策略：

```txt
localStorage key: nonewhite_auth_token
```

选择原因：当前项目是前后端分离 SPA，没有 cookie/session 基础设施；该策略便于 Phase 2 联调。后续如果改成 HttpOnly cookie，必须作为单独安全改造任务，不能在某个页面里局部修改。

认证状态统一字段：

```js
const authToken = ref('')
const currentUser = ref(null)
const authStatus = ref('idle')
const authErrorMessage = ref('')
```

store 必须提供的动作：

```txt
loadTokenFromStorage
saveToken
clearAuth
loginWithCredentials
registerWithCredentials
loadCurrentUser
logout
```

退出登录规则：

- 前端必须清除 `nonewhite_auth_token`。
- 前端必须清空 `currentUser`。
- 如果后端没有 logout endpoint，退出登录是纯前端行为，不要伪造 `POST /api/auth/logout` 已完成。

### 15.4 页面实现框架

`LoginView.vue` 必须包含：

- `email`
- `password`
- `status`
- `errorMessage`
- submit handler 调用 `login(payload)` 或 store action
- 成功后跳转到 `/profile` 或保持在当前页并展示成功，具体行为必须在任务中写清楚

`RegisterView.vue` 必须包含：

- `username`
- `email`
- `password`
- `confirmPassword`，仅前端使用，不发送给后端
- `status`
- `errorMessage`
- submit handler 调用 `register(payload)`

`ProfileView.vue` 必须包含：

- 未登录状态提示。
- 已登录时展示 `currentUser.username`、`currentUser.email`、`currentUser.avatarUrl`、`currentUser.role`。
- 资料编辑表单。
- 修改密码表单。
- 头像区域可展示 `currentUser.avatarUrl` 或占位；如果前端尚未接入 `POST /api/users/me/avatar`，必须显示“待接入”状态且不发起上传请求。
- 如果接入头像上传，必须使用 `multipart/form-data` 字段 `avatar`、携带 `Authorization: Bearer <token>`，成功后刷新 `currentUser.avatarUrl`。
- 收藏列表选项卡 UI 占位，Phase 2 只展示空状态或“Phase 4 接入收藏数据”，不能请求收藏 API，不能标记 Phase 4 收藏数据完成。

### 15.5 路由实现框架

Phase 2 推荐路由：

```js
{
  path: '/login',
  name: 'login',
  component: LoginView,
  meta: { label: '登录', guestOnly: true },
}
```

```js
{
  path: '/register',
  name: 'register',
  component: RegisterView,
  meta: { label: '注册', guestOnly: true },
}
```

```js
{
  path: '/profile',
  name: 'profile',
  component: ProfileView,
  meta: { label: '个人中心', requiresAuth: true },
}
```

如果 Header 自动渲染所有 routes，需要决定：

- 登录后是否隐藏“登录/注册”。
- 未登录是否隐藏“个人中心”。
- 这个逻辑由 `AppHeader.vue` 统一处理，不要在每个页面里处理导航。

### 15.6 前端测试和手动验收框架

当前项目未引入前端测试框架。角色 B 有两个选择：

1. 不新增测试框架，只做 build + 浏览器手动验收，并在交接中写清楚。
2. 引入 Vitest/Vue Test Utils，但必须单独说明依赖、脚本和测试范围。

未引入测试框架时，最小验收清单：

| 页面 | 操作 | 期望 |
|---|---|---|
| `/login` | 输入空邮箱提交 | 显示前端或后端错误 |
| `/login` | 输入错误密码 | 显示 `Invalid email or password` |
| `/register` | 两次密码不一致 | 前端阻止提交 |
| `/register` | 合法数据注册 | 显示成功或跳转 |
| `/profile` | 未登录访问 | 显示未登录或跳转登录 |
| `/profile` | 登录后访问 | 展示当前用户信息 |

所有页面改动后必须确认 `/` 和 `/test-api` 没有被破坏。

---

## 16. 输入校验规则

后端是最终校验来源，前端只做体验增强。

Phase 2 默认校验建议：

| 字段 | 后端规则 | 前端规则 |
|---|---|---|
| `username` | 3-32 字符，只允许字母、数字、下划线或项目确认的字符集，唯一 | 即时提示长度和非法字符 |
| `email` | 必须是邮箱格式，保存前转小写，唯一 | 即时提示邮箱格式 |
| `password` | 至少 8 字符 | 即时提示长度 |
| `currentPassword` | 非空 | 非空提示 |
| `newPassword` | 至少 8 字符，不能和旧密码相同 | 即时提示长度和一致性 |
| `avatar` | 只允许 `image/png`、`image/jpeg`、`image/webp`，最大 `MAX_AVATAR_SIZE_BYTES`，后端同时校验 MIME 和文件签名 | 上传前提示类型和大小；当前未接入前端上传交互时只显示占位 |

校验失败必须返回统一 envelope。前端只读取 `message` 展示，不解析后端内部错误对象。

---

## 17. 测试和验证规则

每个功能至少覆盖三个场景：

1. Happy path：正常输入成功。
2. Edge case：空值、格式错误、重复数据、token 缺失等。
3. Regression：不破坏已有 `/api/test`、首页、`/test-api`。

### 17.1 后端验证

后端改动后至少运行：

```bash
cargo fmt --manifest-path server/Cargo.toml --check
cargo check --manifest-path server/Cargo.toml
cargo test --manifest-path server/Cargo.toml
```

涉及 API 时，还必须用 curl 或等价方式验证真实接口：

```bash
curl http://127.0.0.1:3000/api/test
```

新增接口必须增加对应 curl 验证说明。

### 17.2 前端验证

前端改动后至少运行：

```bash
npm --prefix client install
npm --prefix client run build
```

涉及页面时，必须实际打开或通过浏览器自动化访问：

```txt
/
/test-api
/login
/register
/profile
```

如果当前环境没有浏览器，必须在交接说明中写明，并至少保留 build 结果和手动验证步骤。

### 17.3 数据库验证

数据库或 compose 改动后至少运行：

```bash
docker compose config
```

如果环境允许，还应运行：

```bash
docker compose up -d
```

migration 工具选定后，必须补充：

```txt
apply migration command
rollback migration command
```

### 17.4 全项目验证

交接前优先运行：

```bash
npm run lint
```

如果 `npm run lint` 因环境问题失败，必须说明失败原因。例如当前已知前端依赖未安装时会出现 `vite: not found`。

---

## 18. README 和文档更新规则

每次完成实际开发后必须检查 README 是否需要更新。

必须更新的情况：

- 新增或删除目录。
- 新增脚本或命令。
- 新增环境变量。
- 新增 API endpoint。
- 新增数据库表或 migration。
- Phase checklist 中某项已经真实完成并验证通过。

禁止：

- 只写了 UI 占位就标记“登录注册完成”。
- 只写了 model 就标记“建表完成”。
- 只写了后端接口但没有前端联调，就标记“前后端完成”。
- README 写不存在的命令。
- 把 mock 数据当成真实联调完成。

完成状态定义：

| 状态 | 可以怎么写 | 不能怎么写 |
|---|---|---|
| 未开始 | `[ ]` | 不要写已完成 |
| 进行中 | 文档说明“已准备 / 待联调” | 不要勾选 |
| 已完成 | `[x]` 并有代码和验证 | 不要无验证勾选 |

### 18.1 JOURNALIST 日志写入规则

`agent/JOURNALIST/**/**_LOG.md` 是跨角色交接的时间线日志，必须只追加、不重写历史。

规则：

- 新增 handoff、QA evidence、known limits 时，只能在对应日志文件末尾追加新区块。
- 禁止删除、重排、改写旧日志内容；旧记录即使有遗漏，也只能追加补充说明或更正说明。
- 禁止把 JWT、密码、JWT secret、真实 `.env` 值、数据库 dump 或其他敏感信息写入日志。
- 每次追加日志必须写清任务 ID、变更文件、验证命令和仍未完成事项。

---

## 19. 每个角色的交接模板

每个 Agent 完成任务后，必须留下交接说明。

```md
## Handoff

Role: A/B/C
Branch: feat/xxx
Task: 简述任务

### Changed Files
- path/to/file: 修改原因

### Contracts Changed
- API:
- DTO:
- DB fields:
- Env vars:

### Verification
- Command: `...`
  Result: passed/failed，关键输出
- Manual QA:
  Result:

### Known Limits
- 尚未完成：
- 需要其他角色接手：

### Conflict Notes
- 修改了哪些共享文件：
- 可能冲突的文件：
```

如果没有交接说明，下一个 Agent 不应直接假设任务完成。

---

## 20. 高风险分歧点和预防规则

| 风险 | 表现 | 预防规则 |
|---|---|---|
| API envelope 漂移 | 有的接口返回 `{ data }`，有的返回 `{ code, data, message }` | 所有接口必须复用 `ApiResponse` |
| 字段命名漂移 | DB 是 `avatar_url`，前端用 `avatar`，API 返回 `avatarUrl` | 每个实体必须维护字段映射表 |
| 错误码漂移 | 不同接口同一错误返回不同 code | 使用本文档错误码范围 |
| token 存储不一致 | 有页面用 localStorage，有页面用内存变量 | 实现前先在契约中确定 token 存储策略 |
| migration 工具混用 | 有 SQL 文件又有 Rust migration | 先选一种，写入 README 后再使用 |
| 共享文件冲突 | 多人同时改 router、routes/mod.rs、README | 共享文件最小修改，交接中注明 |
| README 状态虚高 | 未联调就勾选完成 | 必须有验证命令和结果才能勾选 |
| mock 数据污染 | 前端 mock 字段和真实接口不同 | mock 必须复制契约样例，不得自创字段 |
| 密码泄漏 | API 返回 `password_hash` | `password` 和 `password_hash` 永不出现在响应 DTO |
| 环境变量遗漏 | 本地能跑，别人缺变量 | 新变量必须同步 `.env.example` 和 README |
| 依赖未安装 | build 失败 `vite: not found` | 前端验证前运行 `npm --prefix client install` |

---

## 21. Agent 开工检查清单

开工前复制并逐项确认：

```md
## Start Checklist

- [ ] 已阅读 `README.md`
- [ ] 已阅读 `agent/AGENT_RULES.md`
- [ ] 已阅读 `agent/COLLABORATION_PLAN.md`
- [ ] 已检查当前 Git 分支和工作区状态
- [ ] 已确认自己是角色 A / B / C
- [ ] 已确认要改的文件属于自己的所有权范围
- [ ] 已确认是否涉及共享高冲突文件
- [ ] 已确认接口字段、DTO、数据库字段是否已有契约
- [ ] 已确认需要新增的环境变量
- [ ] 已确认要运行的验证命令
```

如果任何一项无法确认，先补契约或询问项目负责人，不要直接编码。

---

## 22. Phase 2 最小可执行任务队列

为了三个人能同时开始，建议按下面顺序拆任务。

### 22.1 角色 C 优先任务

1. 确认 migration 工具和目录。
2. 写 `users` 表 schema/migration。
3. 更新用户字段映射表。
4. 更新 API contract 文档。
5. 更新 `.env.example` 和 README。

#### C-01：锁定数据库和契约文件结构

输入：

- `README.md` Phase 2 用户系统需求。
- 本文档第 9、10、12、13 节。

要做：

- 决定数据库 migration 目录，默认 `server/migrations/`。
- 决定 API 契约是否单独拆到 `docs/api/phase2-auth.md`。如果拆分，本文档保留摘要，详细契约放 `docs/api/phase2-auth.md`。
- 决定数据库说明是否单独拆到 `docs/database/phase2-users.md`。

交付物：

```txt
server/migrations/          # 如选择 SQL migration
docs/api/phase2-auth.md     # 如拆分 API 契约
docs/database/phase2-users.md
```

验收：

- A 能根据契约写 DTO 和 handler。
- B 能根据契约写 API client 和页面字段。
- README 未把任何未实现功能标记完成。

#### C-02：创建 `users` 表 migration

输入：

- C-01 的 migration 决策。
- 本文档第 13.2 节 schema 草案。

要做：

- 新增 `server/migrations/YYYYMMDDHHMMSS_create_users.sql`。
- 写入 `users` 表字段、唯一约束、索引。
- 如果 migration 工具有 rollback 格式，同步写 rollback。

验收：

- SQL 字段包含 `id`、`username`、`email`、`password_hash`、`avatar_url`、`role`、`created_at`、`updated_at`。
- 字段名和第 9.1 节完全一致。
- `email` 和 `username` 有唯一约束。
- `password_hash` 不允许为空。

#### C-03：补全环境变量契约

输入：

- A 是否选择 JWT。
- A 是否选择数据库连接池。

要做：

- 在根 `.env.example` 和 `server/.env.example` 中加入 `JWT_SECRET`、`JWT_EXPIRES_IN_SECONDS`。
- 如果实现头像上传，再加入 `UPLOAD_DIR`、`MAX_AVATAR_SIZE_BYTES`。
- README 本地启动说明增加这些变量说明。

验收：

- `.env.example` 不包含真实 secret。
- `server/.env.example` 和根 `.env.example` 不冲突。
- A 可以直接从 env 读取变量。

#### C-04：联调验收脚本说明

输入：

- A 已实现的 API。
- B 已接入的页面。

要做：

- 在文档中维护 curl 验收命令。
- 记录每个接口的成功和失败样例。
- 检查 API JSON 字段和前端变量一致。

验收：

- 至少包含 register、login、me 三类 curl。
- 每个 curl 都写明预期 `code`。
- 失败场景至少覆盖 invalid email、wrong password、missing token。

### 22.2 角色 A 优先任务

1. 添加数据库连接配置读取。
2. 添加用户 DTO。
3. 添加注册请求校验测试。
4. 实现注册接口。
5. 添加登录请求校验测试。
6. 实现登录和 JWT。
7. 添加认证中间件。
8. 实现当前用户和资料更新接口。

#### A-01：建立后端应用状态和配置

输入：

- C-03 环境变量契约。
- 当前 `server/src/config.rs` 和 `server/src/main.rs`。

要改文件：

```txt
server/src/config.rs
server/src/main.rs
server/src/db.rs          # 新增，如接入数据库
server/src/state.rs       # 可选；如果 AppState 简单，可放 main.rs 或 lib-like module
server/Cargo.toml
```

要做：

- 定义 `AppConfig`、`ServerConfig`、`DatabaseConfig`、`AuthConfig`。
- 读取 `DATABASE_URL`、`JWT_SECRET`、`JWT_EXPIRES_IN_SECONDS`。
- 明确后端 env 加载优先级：`server/.env` 优先于根目录 `.env`。
- 创建数据库连接池。
- 创建 `AppState` 并注入 router。

验收：

- `cargo check` 通过。
- 缺少必须 env 时后端启动失败信息明确，但不泄漏 secret。
- `GET /api/test` 仍可用。

#### A-02：建立错误和响应框架

输入：

- 当前 `server/src/response.rs`。
- 本文档第 8、14.6 节。

要改文件：

```txt
server/src/response.rs
server/src/error.rs
server/src/main.rs
```

要做：

- 保留 `ApiResponse<T>`。
- 新增 `AppError`。
- 实现 `IntoResponse` 或等价转换。
- 保证 fallback 仍返回统一 envelope。

验收：

- 404 fallback 必须返回 HTTP 404，body 使用统一 envelope，`code=40400`，`data=null`，`message="API endpoint not found"`。
- 后续 handler 可以返回 `Result<_, AppError>`。

#### A-03：实现 User model、DTO、repository

输入：

- C-02 migration。
- 第 9.1 节字段映射表。

要新增文件：

```txt
server/src/models/mod.rs
server/src/models/user.rs
server/src/dto/mod.rs
server/src/dto/users.rs
server/src/dto/auth.rs
server/src/repositories/mod.rs
server/src/repositories/user_repository.rs
```

要做：

- `UserRow` 表示数据库字段。
- `UserResponse` 表示 API 输出字段。
- 实现 `From<UserRow> for UserResponse` 或等价转换函数。
- repository 提供 `find_user_by_email`、`find_user_by_username`、`create_user`。

验收：

- `UserResponse` JSON 是 `avatarUrl`、`createdAt`、`updatedAt`。
- `password_hash` 不进入 response DTO。
- repository 不负责 HTTP 状态码。

#### A-04：注册接口 TDD 和实现

输入：

- 第 10.1 节注册契约。
- A-01、A-02、A-03。

要改文件：

```txt
server/src/routes/auth.rs
server/src/routes/mod.rs
server/src/services/auth_service.rs
server/tests/auth_api.rs
```

先写测试：

```txt
register_success_returns_user
register_rejects_invalid_email
register_rejects_short_password
register_rejects_duplicate_email
```

再实现：

- `POST /api/auth/register`。
- username/email/password 校验。
- email 保存前小写。
- bcrypt hash。
- 插入用户。
- 返回 `UserResponse`。

验收：

- 注册成功不返回 token，除非 C/A/B 明确改契约。
- 注册响应不包含 `password_hash`。
- 重复邮箱返回 409。

#### A-05：登录接口 TDD 和实现

输入：

- 第 10.2 节登录契约。
- A-04 用户创建能力。

测试：

```txt
login_success_returns_token_and_user
login_rejects_invalid_email
login_rejects_wrong_password
```

实现：

- `POST /api/auth/login`。
- 根据 email 查用户。
- bcrypt 校验密码。
- JWT 签发。
- 返回 `AuthTokenResponse`。

验收：

- 错误邮箱和错误密码统一返回 `Invalid email or password`。
- `tokenType` 固定 `Bearer`。
- `expiresIn` 单位是秒。

#### A-06：认证中间件和当前用户接口

输入：

- A-05 token 签发。
- 第 10.3 节契约。

要新增/修改：

```txt
server/src/middleware/mod.rs
server/src/middleware/auth.rs
server/src/routes/users.rs
server/src/services/user_service.rs
server/tests/users_api.rs
```

实现：

- 从 `Authorization` header 读取 Bearer token。
- 校验 token。
- 提取 user id。
- `GET /api/users/me` 返回当前用户。

验收：

- 无 token 返回 401。
- 无效 token 返回 401。
- 有效 token 返回当前用户。

#### A-07：资料更新和密码修改

输入：

- A-06 当前用户能力。
- 第 10.4、10.5 节契约。

实现：

- `PATCH /api/users/me`。
- `PATCH /api/users/me/password`。
- 只允许更新契约允许字段。
- `PATCH /api/users/me` 不能直接更新 `avatarUrl`；头像地址只能由头像上传接口写入。
- 修改密码必须校验旧密码。

验收：

- 未登录不能更新。
- username 冲突返回 409。
- 直接提交 `avatarUrl` 必须返回 HTTP 400，`code=40005`，不能写入数据库。
- 旧密码错误返回 HTTP 401，`code=40104`，`message="Current password is incorrect"`。
- 修改成功后旧密码不可登录，新密码可登录。

### 22.3 角色 B 优先任务

1. 确认前端依赖安装并 build 通过。
2. 添加 `client/src/api/auth.js`。
3. 添加 `client/src/api/users.js`。
4. 添加登录页面。
5. 添加注册页面。
6. 添加认证状态管理。
7. 添加个人中心页面。
8. 接入真实后端接口并处理错误提示。

#### B-01：安装依赖并确认前端基线

输入：

- 当前 `client/package.json`。

要做：

```bash
npm --prefix client install
npm --prefix client run build
```

验收：

- build 通过。
- 如果 build 不通过，只修复和依赖/构建直接相关的问题，不改业务范围。
- `/` 和 `/test-api` 当前页面不被改坏。

#### B-02：建立统一 HTTP client

输入：

- 第 8 节 API envelope。
- 当前 `client/src/api/test.js`。

要新增/修改：

```txt
client/src/api/http.js
client/src/api/test.js
```

要做：

- 新增 `requestJson(path, options)`。
- 统一处理 `response.ok` 和 `body.code !== 0`。
- 改造 `fetchTestStatus()` 复用 `requestJson` 或保持兼容但不得重复复杂逻辑。

验收：

- `/test-api` 仍可调用 `GET /api/test`。
- 错误信息优先使用后端 `message`。

#### B-03：实现 auth API client

输入：

- 第 10.1、10.2 节契约。
- B-02 `requestJson`。

要新增：

```txt
client/src/api/auth.js
```

函数：

```txt
register({ username, email, password })
login({ email, password })
logoutLocal()
```

验收：

- 请求字段和契约完全一致。
- 不发送 `confirmPassword`。
- 返回值直接是 `body.data`。

#### B-04：实现 users API client

输入：

- 第 10.3、10.4、10.5、10.6 节契约。
- token header 规则。

要新增：

```txt
client/src/api/users.js
```

函数：

```txt
fetchCurrentUser(authToken)
updateCurrentUser(payload, authToken)
changePassword(payload, authToken)
uploadAvatar(file, authToken)
```

验收：

- 所有需要登录的函数都带 `Authorization: Bearer <token>`。
- `uploadAvatar` 如实现，必须使用 `FormData` 字段 `avatar`，请求头只追加 auth，不手动伪造 `Content-Type`，也不写假 URL。

#### B-05：实现认证状态管理

输入：

- 第 15.3 节 token 存储策略。

要新增：

```txt
client/src/stores/auth.js
```

如果引入 Pinia：

- 修改 `client/package.json`。
- 修改 `client/src/main.js` 注册 Pinia。
- README 写明 Pinia 已引入。

如果不引入 Pinia：

- `auth.js` 可以导出 Vue `ref` 和函数组成的轻量 store。
- 必须在文件顶部注释说明“Phase 2 暂不引入 Pinia”。

验收：

- `authToken` 从 `localStorage` key `nonewhite_auth_token` 读取。
- `logout()` 清除 token 和 `currentUser`。
- 不在多个文件重复操作 localStorage key。

#### B-06：实现登录和注册页面

输入：

- B-03 auth API client。
- B-05 auth store。

要新增/修改：

```txt
client/src/views/LoginView.vue
client/src/views/RegisterView.vue
client/src/router/index.js
client/src/components/AppHeader.vue  # 如需处理导航显示
client/src/style.css
```

验收：

- `/login` 可访问。
- `/register` 可访问。
- 表单字段和契约一致。
- `confirmPassword` 不发送到后端。
- loading/error/success 状态明确。
- build 通过。

#### B-07：实现个人中心页面

输入：

- B-04 users API client。
- B-05 auth store。

要新增/修改：

```txt
client/src/views/ProfileView.vue
client/src/components/profile/ProfileForm.vue
client/src/components/profile/PasswordForm.vue
client/src/components/profile/AvatarUploader.vue
client/src/components/profile/FavoritesPlaceholder.vue
client/src/router/index.js
```

验收：

- `/profile` 可访问。
- 未登录状态明确。
- 已登录时能展示用户信息。
- 修改资料和修改密码有明确错误提示。
- 前端头像上传未接入时只显示“待接入”，不伪造成功；若接入上传，必须调用真实后端头像 API 并刷新头像展示。
- 收藏列表选项卡只做 UI 占位，显示 Phase 4 接入说明，不请求收藏接口。

### 22.4 三人联调任务

#### INT-01：注册闭环

负责人：A/B/C 共同，C 记录结果。

步骤：

1. 启动 PostgreSQL。
2. 应用 migration。
3. 启动后端。
4. 启动前端。
5. 在 `/register` 注册新用户。
6. 用数据库或 API 确认用户已创建。

验收：

- 页面显示成功。
- API 返回 `code=0`。
- 数据库有用户记录。
- 响应中没有 `password_hash`。

#### INT-02：登录和当前用户闭环

步骤：

1. 用已注册用户登录。
2. 确认前端保存 token。
3. 访问 `/profile`。
4. 刷新页面后仍能加载当前用户。

验收：

- 登录返回 `token`、`tokenType`、`expiresIn`、`user`。
- `/profile` 展示的是同一个用户。
- token 无效时显示未登录或错误提示。

#### INT-03：回归检查

步骤：

1. 访问 `/`。
2. 访问 `/test-api`。
3. 调用 `GET /api/test`。
4. 运行项目验证命令。

验收：

- 首页仍正常。
- `/test-api` 仍能显示后端测试结果。
- `GET /api/test` 返回 `status: ok`。
- `npm run lint` 或拆分验证命令通过；如果因环境失败，记录原因。

---

## 23. 停止条件

出现以下情况必须停止当前实现，先修正计划或沟通：

- README 和实际代码状态严重不一致。
- 同一 API 字段在 DB/Rust/API/frontend 有两个命名版本。
- 不知道接口应该返回 `{}`、`[]` 还是 `null`。
- token 存储策略未定。
- migration 工具未定但任务要求建表。
- 头像上传存储策略未定但任务要求实现上传。
- 需要修改其他角色所有的文件，但没有交接说明。
- 验证命令无法运行，且没有记录原因和替代验证方式。

---

## 24. 最终完成定义

一个任务只有同时满足下面条件，才能被认为完成：

- 代码或文档实现了任务要求。
- 没有超出 README 的产品范围。
- 字段命名符合本文档映射。
- API 返回符合 `{ code, data, message }`。
- README 状态没有虚假勾选。
- 相关验证命令已运行并记录结果。
- 涉及页面或接口时，已经做真实表面验证。
- 交接说明完整。
- 没有留下未说明的共享文件冲突。
