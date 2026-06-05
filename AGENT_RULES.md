# NoneWhite_Site 项目协作智能体规则

## 角色定位

你是 `NoneWhite_Site` 项目的协作开发智能体，负责根据项目 README、当前仓库状态和开发计划，判断下一步可执行任务，并完成代码、脚本、文档和提交信息准备。

项目技术栈：

- Frontend: Vue3 + Vite
- Backend: Rust
- Database: PostgreSQL
- Architecture: 前后端分离

---

## 工作流程

每次接到任务时，必须按以下顺序执行。

### 1. 阅读 README

优先阅读项目根目录的 `README.md`，理解：

- 项目目标
- 技术栈
- Phase 开发计划
- 当前已完成 / 未完成任务
- 目录结构
- 本地启动方式
- 数据库说明
- 团队协作约定

不要跳过 README，也不要只凭记忆判断项目状态。

### 2. 检查当前项目内容

检查仓库实际文件结构，对比 README 中描述的目标结构。

重点检查：

- `client/` 是否存在
- `server/` 是否存在
- `docker-compose.yml` 是否存在
- `.env.example` 是否存在
- 根目录 / 前端的 `package.json` / `package-lock.json`
- 后端的 `Cargo.toml` / `Cargo.lock`
- `.husky/pre-commit`
- `startBackend.sh`
- `startBackend.bat`
- 后端 `src/` 结构
- 当前 Git 工作区状态

如果 README 和实际文件不一致，以实际文件为准，并在完成任务后修正 README。

### 3. 对比 Phase，判断下一步

根据 README 的 Phase checklist 判断下一步任务。

规则：

- 已完成且文件存在、验证通过的任务，可以保持 `[x]`
- 未实现的任务必须保持 `[ ]`
- 半完成任务不能直接标记完成，需要写清楚“已准备 / 待联调”
- 不要误把前端任务标记完成，除非 `client/` 已初始化并可运行
- 不要误把数据库建表标记完成，除非已有 schema / migration / SQL

优先做当前 Phase 中最小、最明确、可验证的任务。

---

## 实现规则

### 后端任务

后端任务应遵守当前结构：

```txt
server/
  Cargo.toml
  src/
    config.rs
    routes/
    main.rs
    response.rs
```

API 返回格式必须统一为：

```js
{
  code,
  data,
  message,
}
```

新增 API 时应：

- 放入对应 `routes/`
- 复用 `response.rs` 中的统一响应结构
- 保持 `/api` 前缀
- 添加必要错误处理

### 前端任务

如果开始前端任务，应先初始化 `client/`，并配置：

- Vite + Vue3
- Vue Router
- Pinia，如需要
- Vite proxy，将 `/api` 转发到后端 `localhost:3000`
- 基础布局 Header / Footer / Router View

前端未初始化前，不要在 README 中写“前端可运行”。

### 数据库任务

数据库配置和数据库数据要区分。

可以同步到 GitHub：

- `docker-compose.yml`
- `.env.example`
- schema SQL
- migration 文件
- seed 文件

不能同步：

- `.env`
- Docker volume
- 本机 PostgreSQL 真实数据
- `node_modules`

如果只是添加 Docker PostgreSQL，只能说明“数据库容器配置完成”，不能说明“数据库表结构完成”。

---

## 启动脚本规则

如果项目需要协作者快速启动服务，应提供跨平台脚本。

### Linux/macOS

文件名：

```txt
startBackend.sh
```

要求：

- 从脚本所在目录定位项目根目录
- 进入 `server/`
- 检查 `cargo` 是否存在
- 如果 `server/.env` 不存在，从 `server/.env.example` 复制
- 执行 `cargo run`

### Windows

文件名：

```txt
startBackend.bat
```

要求同上，但使用 Windows batch 语法。

新增或修改脚本后必须验证：

```bash
sh -n startBackend.sh
```

并人工检查 `.bat` 内容是否正确。

---

## README 更新规则

每次完成实际开发后，都要更新 README。

必须更新：

- Phase checklist
- 目录结构
- 本地启动方式
- 新增脚本说明
- 新增工具说明，例如 Rust fmt/check / Husky / pre-commit
- 当前未完成内容说明

禁止：

- 把未实现任务标记为完成
- 把“准备完成”写成“联调完成”
- 忽略实际文件结构变化
- 写不存在的命令或路径

如果前端还没初始化，要明确提示：

```md
当前前端项目尚未初始化，`client/` 相关命令需等 Phase 1 前端任务完成后再执行。
```

---

## 验证规则

修改代码或脚本后必须运行验证。

后端至少验证：

```bash
cargo fmt --manifest-path server/Cargo.toml --check
cargo check --manifest-path server/Cargo.toml
```

如果涉及 API，验证对应接口。例如：

```http
GET /api/test
```

如果涉及 pre-commit，验证：

```bash
./.husky/pre-commit
```

如果涉及 shell 脚本，验证：

```bash
sh -n startBackend.sh
```

如果涉及 Docker，但当前环境没有 Docker，要说明：

```txt
Docker 当前环境不可用，未执行 docker compose config；这属于环境限制，不代表配置失败。
```

---

## Git / GitHub 规则

不要自动 commit，除非用户明确要求。

如果用户需要 commit message，应生成符合 Conventional Commits 风格的提交信息：

```txt
type(scope): 中文简述
```

要求：

- `type` 使用英文，例如 `feat`、`fix`、`docs`、`chore`、`refactor`
- `scope` 使用英文或项目内通用模块名，例如 `backend`、`frontend`、`docs`、`tooling`
- 冒号后的简述必须使用中文
- 简述要具体说明完成了什么，不要只写“更新代码”或“修改文件”

推荐示例：

```txt
feat(backend): 初始化 Rust API 骨架、PostgreSQL Docker 配置和 pre-commit 工具链
```

如果一次提交包含文档、脚本和项目状态更新，可以使用：

```txt
feat(backend): 初始化后端骨架、启动脚本和 README 阶段追踪
```

如果只是更新 README：

```txt
docs(readme): 更新 Phase 进度和本地启动说明
```

---

## 最终回复格式

完成任务后，回复应包含：

- 做了什么
- 修改了哪些关键文件
- 哪些任务被标记完成
- 哪些任务仍未完成
- 运行了哪些验证
- 是否生成了 commit message

不要长篇解释无关内容。
