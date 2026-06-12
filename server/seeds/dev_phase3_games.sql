-- Development seed data for Phase 3 game browsing.
-- Safe for local/dev environments only; do not treat as production initialization data.

INSERT INTO categories (id, name, slug) VALUES
  (1, '剧情', 'story'),
  (2, '悬疑', 'mystery'),
  (3, '恋爱', 'romance'),
  (4, '奇幻', 'fantasy')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, slug = EXCLUDED.slug;

INSERT INTO tags (id, name, slug) VALUES
  (1, '校园', 'school'),
  (2, '治愈', 'healing'),
  (3, '多结局', 'multi-ending'),
  (4, '悬疑', 'suspense'),
  (5, '幻想', 'fantasy')
ON CONFLICT (id) DO UPDATE SET name = EXCLUDED.name, slug = EXCLUDED.slug;

INSERT INTO games (
  id, title, developer, publisher, release_date, description,
  cover_url, category_id, search_text, likes_count, favorites_count
) VALUES
  (1, '雨后第七封信', 'NoneWhite Studio', 'NoneWhite', '2024-01-18', '一段发生在旧教学楼里的短篇视觉小说，围绕失物、信件与毕业前夜展开。', '', 1, '雨后第七封信 NoneWhite Studio 剧情 校园 治愈 多结局', 128, 46),
  (2, '白塔回声', 'Amber Loop', 'NoneWhite', '2024-04-02', '探索封闭白塔中的记忆房间，拼合角色之间被抹去的约定。', '', 2, '白塔回声 Amber Loop 悬疑 多结局', 92, 31),
  (3, '薄荷色夏日', 'Mint Days', 'Indie Harbor', '2023-08-12', '以社团活动为主线的轻恋爱故事，包含多条角色支线与夏日祭章节。', '', 3, '薄荷色夏日 Mint Days 恋爱 校园 治愈 多结局', 210, 83),
  (4, '星屑档案室', 'Orbit Type', 'NoneWhite', '2025-02-26', '在漂浮档案馆中检索星球文明的最后记录，选择会影响档案修复顺序。', '', 4, '星屑档案室 Orbit Type 奇幻 幻想 多结局', 76, 29),
  (5, '午后三点的侦探', 'Clockwork Note', 'Indie Harbor', '2023-11-09', '小镇咖啡馆里的日常推理合集，每个案件都藏着角色关系的新线索。', '', 2, '午后三点的侦探 Clockwork Note 悬疑 治愈', 154, 55),
  (6, '月台尽头', 'Silent Rail', 'NoneWhite', '2024-09-21', '末班车停靠在不存在的月台，玩家需要在循环中找回乘客的名字。', '', 1, '月台尽头 Silent Rail 剧情 悬疑 幻想', 188, 67)
ON CONFLICT (id) DO UPDATE SET
  title = EXCLUDED.title,
  developer = EXCLUDED.developer,
  publisher = EXCLUDED.publisher,
  release_date = EXCLUDED.release_date,
  description = EXCLUDED.description,
  cover_url = EXCLUDED.cover_url,
  category_id = EXCLUDED.category_id,
  search_text = EXCLUDED.search_text,
  likes_count = EXCLUDED.likes_count,
  favorites_count = EXCLUDED.favorites_count,
  updated_at = NOW();

INSERT INTO game_tags (game_id, tag_id) VALUES
  (1, 1), (1, 2), (1, 3),
  (2, 4), (2, 3),
  (3, 1), (3, 2), (3, 3),
  (4, 5), (4, 3),
  (5, 4), (5, 2),
  (6, 4), (6, 5)
ON CONFLICT (game_id, tag_id) DO NOTHING;

INSERT INTO screenshots (id, game_id, url, sort_order) VALUES
  (101, 1, '', 1),
  (102, 1, '', 2),
  (201, 2, '', 1),
  (401, 4, '', 1),
  (402, 4, '', 2),
  (403, 4, '', 3),
  (501, 5, '', 1),
  (601, 6, '', 1)
ON CONFLICT (id) DO UPDATE SET
  game_id = EXCLUDED.game_id,
  url = EXCLUDED.url,
  sort_order = EXCLUDED.sort_order;

SELECT setval('categories_id_seq', GREATEST((SELECT MAX(id) FROM categories), 1), true);
SELECT setval('tags_id_seq', GREATEST((SELECT MAX(id) FROM tags), 1), true);
SELECT setval('games_id_seq', GREATEST((SELECT MAX(id) FROM games), 1), true);
SELECT setval('screenshots_id_seq', GREATEST((SELECT MAX(id) FROM screenshots), 1), true);
