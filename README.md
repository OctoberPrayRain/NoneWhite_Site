# NoneWhite_Site 使用说明

这份文档面向本地运行和功能使用。开发计划、数据库表结构和 API 细节见根目录 `DEVELOP.md`。

## 功能概览

NoneWhite_Site 是一个文件与资源分享站。启动后可以完成以下操作：

- 浏览文件列表和文件详情。
- 按关键词、分类和标签筛选资源。
- 注册、登录、退出登录。
- 管理个人资料、修改密码、上传头像。
- 收藏资源、点赞、发表评论和回复。
- 查看资源下载信息。
- 管理员可进入后台维护资源、预览图、下载链接和评论。

## 运行前准备

需要先安装：

- Node.js 和 npm。
- Rust 与 Cargo。
- PostgreSQL，或可用的 Docker / Docker Compose。

首次运行建议在项目根目录执行：

```bash
cp .env.example .env
cp server/.env.example server/.env
```

默认 `.env.example` 只适合本地开发。正式部署前必须替换数据库密码、`JWT_SECRET` 等敏感配置，不要把真实配置提交到仓库。

## 本地快速启动

在项目根目录按顺序打开三个终端执行。

### 1. 初始化数据库

Linux / macOS：

```bash
./setupDatabase.sh
```

Windows：

```bat
setupDatabase.bat
```

脚本会创建缺失的 `.env` 文件，启动或连接 PostgreSQL，按顺序执行 migrations，并导入本地开发 seed 数据。

如果要强制使用 Docker：

```bash
DB_SETUP_DRIVER=docker ./setupDatabase.sh
```

如果要使用本机 PostgreSQL：

```bash
DB_SETUP_DRIVER=local ./setupDatabase.sh
```

### 2. 启动后端

Linux / macOS：

```bash
./startBackend.sh
```

Windows：

```bat
startBackend.bat
```

后端默认监听：`http://127.0.0.1:3000`。

### 3. 启动前端

Linux / macOS：

```bash
./startFrontend.sh
```

Windows：

```bat
startFrontend.bat
```

前端默认访问：`http://127.0.0.1:5173`。

前端开发服务器已配置代理，页面中的 `/api` 和 `/uploads` 请求会转发到后端，无需额外配置 CORS。

## 常用页面

| 页面 | 地址 | 说明 |
|---|---|---|
| 首页 | `/` | 查看站点入口和功能介绍 |
| 文件列表 | `/files` | 浏览资源、分类筛选、标签筛选 |
| 文件详情 | `/files/:id` | 查看简介、预览图、互动区和下载信息 |
| 搜索 | `/search` | 通过标题、提供方、发布方或标签搜索 |
| 注册 | `/register` | 创建普通用户账号 |
| 登录 | `/login` | 登录后访问个人中心、提交和互动功能 |
| 个人中心 | `/profile` | 修改资料、密码、头像，查看收藏 |
| 提交文件 | `/submit-file` | 提交新的文件资源，等待审核 |
| 管理后台 | `/admin` | 管理员维护资源、下载链接和评论 |

## 普通用户使用流程

1. 打开 `http://127.0.0.1:5173`。
2. 点击“注册”创建账号，或点击“登录”进入已有账号。
3. 进入“文件列表”浏览资源，可使用分类和标签筛选。
4. 进入“文件详情”查看简介、预览图、下载信息和评论。
5. 登录后可点赞、收藏、发表评论或回复。
6. 在“个人中心”维护资料、头像和收藏列表。
7. 在“提交文件”填写资源信息（支持上传封面、预览图和资源文件），提交后等待管理员审核。

## 管理员使用流程

管理员账号需要数据库中用户记录的 `role` 为 `admin`。登录管理员账号后访问 `/admin`。

后台可执行：

- 创建、编辑、删除资源条目。
- 上传封面、预览图，以及为下载链接上传资源文件。
- 为资源维护下载链接。
- 查看并删除评论。
- 审核用户提交的资源。

不要在公开文档、提交记录或协作日志中写入真实下载地址、生产密码或真实 token。

## 常见问题

### 数据库启动失败

先确认 PostgreSQL 或 Docker 可用。Docker 方式需要 Docker daemon 正在运行，并且当前用户有权限执行 `docker`。

也可以改用本机 PostgreSQL：

```bash
DB_SETUP_DRIVER=local ./setupDatabase.sh
```

### 端口被占用

默认端口是：

- 后端：`3000`
- 前端：`5173`
- PostgreSQL：`5432`

如需调整后端或数据库端口，修改根目录 `.env` 和 `server/.env` 后重新启动对应服务。

### 前端页面接口报 502 或请求失败

通常是后端没有启动，或 Vite 代理无法连接 `127.0.0.1:3000`。先确认后端终端没有报错，再刷新前端页面。

### 头像或图片上传后无法访问

确认后端正在运行，并检查 `UPLOAD_DIR` 与 `UPLOAD_PUBLIC_BASE_URL`。默认上传目录位于 `server/uploads/`，该目录用于本地运行，不应提交真实上传文件。

### 资源文件的上传与下载

资源文件均通过应用界面（用户 `/submit-file` 或后台 `/admin`）上传。新上传的资源会由后端写入 `OPENLIST_RESOURCE_UPLOAD_DIR` 指定的 OpenList 目录，目录格式为 `openlist:/GoogleDrive/...`，并且该目录需要提前在 OpenList 中创建。后端需要配置 `OPENLIST_BASE_URL` 与 `OPENLIST_TOKEN`，缺少配置或 OpenList 上传失败时会直接返回上传失败，不会回退到本地资源目录。

下载资源文件时，必须通过资源详情页提供的合法下载链接进行访问（由后端统一接口 `/api/games/{gameId}/download-links/{id}/download` 控制）。旧数据中已有的 `/uploads/resources/...` 内部标记仍可通过受控下载接口访问，但新资源上传不会再持久写入该目录。

## 验证命令

修改代码或文档后，可运行：

```bash
npm run lint
```

该命令会执行 Rust 格式检查、Rust 编译检查和前端生产构建。
