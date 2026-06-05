<template>
  <section class="hero-section">
    <div class="eyebrow">Phase 1 · Project Skeleton</div>
    <h1>空白分享站前端骨架已就绪</h1>
    <p class="hero-copy">
      当前阶段完成 Vite + Vue3 初始化、基础布局、路由框架和后端测试接口联调入口，
      为后续用户系统、游戏浏览和互动功能打好基础。
    </p>
    <div class="hero-actions">
      <RouterLink class="primary-button" to="/test-api">验证后端 API</RouterLink>
      <a class="secondary-button" href="https://vuejs.org/" target="_blank" rel="noreferrer">
        Vue 文档
      </a>
    </div>
  </section>

  <section class="feature-grid" aria-label="Phase 1 前端能力">
    <article class="feature-card">
      <span>01</span>
      <h2>Vue3 前端项目</h2>
      <p>使用 Vite 初始化，保留轻量的 JavaScript 结构。</p>
    </article>
    <article class="feature-card">
      <span>02</span>
      <h2>基础布局</h2>
      <p>Header、Footer 与页面容器已经抽离为可扩展框架。</p>
    </article>
    <article class="feature-card">
      <span>03</span>
      <h2>路由框架</h2>
      <p>当前仅包含首页和联调验证页，避免提前引入后续阶段功能。</p>
    </article>
    <article class="feature-card">
      <span>04</span>
      <h2>代理联调</h2>
      <p>前端通过 <code>/api</code> 代理访问 Rust 后端。</p>
    </article>
  </section>
</template>
