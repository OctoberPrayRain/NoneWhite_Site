# NoneWhite_Site — 空白分享站

> Galgame 分享交流社区项目 · 开发中

<p align="center">
  <img src="https://img.shields.io/badge/Status-开发中-yellow" alt="开发中" />
  <img src="https://img.shields.io/badge/Frontend-Vue3-4FC08D" alt="Vue3" />
  <img src="https://img.shields.io/badge/Backend-Express-000000" alt="Express" />
  <img src="https://img.shields.io/badge/Database-MySQL-4479A1" alt="MySQL" />
  <img src="https://img.shields.io/badge/License-MIT-yellow" alt="MIT" />
</p>

---

## 项目简介

一个 Galgame 分享站，用户可以浏览 Galgame 详情页、点赞评论、获取网盘下载链接。

**技术栈**：Vue3 + Express + MySQL  
**架构**：前后端分离

---

## 开发计划

> ✅ = 可前后端并行开发

### Phase 1 — 项目骨架（打基础）

- [ ] 初始化 Vite + Vue3 前端项目
- [ ] 初始化 Express 后端项目
- [ ] MySQL 建库 & docker-compose.yml
- [ ] 配置 .env、ESLint、gitignore
- [ ] 配置 Vite proxy（前端请求 `/api` 自动转发到后端）
- [ ] 搭建前端基础布局（Header / Footer / 路由框架）
- [ ] 约定 API 返回格式（统一 `{ code, data, message }`）
- [ ] ✅ 前后端联调验证（写一个 test API）

### Phase 2 — 用户系统（前后端可并行）

**后端：**
- [ ] User Model + 建表
- [ ] 注册 / 登录 API（JWT + bcrypt）
- [ ] 认证中间件
- [ ] 获取 / 更新个人资料 API
- [ ] 修改密码 API
- [ ] 头像上传 API

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
├── client/                    # 前端 Vue3
│   └── src/
│       ├── api/               # 请求封装
│       ├── components/        # 公共组件
│       ├── router/            # 路由
│       ├── stores/            # Pinia
│       ├── views/             # 页面
│       ├── App.vue
│       └── main.js
│
├── server/                    # 后端 Express
│   └── src/
│       ├── config/            # 配置
│       ├── controllers/       # 控制器
│       ├── middleware/        # 中间件
│       ├── models/            # Model
│       ├── routes/            # 路由
│       ├── app.js
│       └── server.js
│
├── docker-compose.yml
└── .env.example
```

---

## 数据库设计（初版）

| 表名 | 主要字段 | 说明 |
|---|---|---|
| `users` | id, username, email, password_hash, avatar, role | 用户 |
| `games` | id, title, developer, publisher, release_date, description, cover_url, category_id, search_text, likes_count, favorites_count | 游戏 |
| `categories` | id, name, slug | 分类 |
| `tags` | id, name | 标签 |
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
cd server
npm install
cp .env.example .env    # 填数据库配置
npm run dev             # → localhost:3000

# 前端（Vite proxy 已将 /api 请求转发到后端，无需处理 CORS）
cd client
npm install
npm run dev             # → localhost:5173
```

---

## 团队分工

| 角色 | 负责内容 |
|---|---|
| 前端 | 页面开发、组件封装、前后端联调 |
| 后端 | API 开发、数据库、认证鉴权 |
| UI/设计 | 界面风格、交互设计 |
| 文档/测试 | API 文档、功能测试 |

---

## License

[MIT](LICENSE)
