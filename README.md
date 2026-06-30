# 屏绪 · mood-wallpaper-app

一款面向 00后 / 10后 的「每日情绪手机主题」App。选一个今天的情绪，App 返回一整套手机主题（锁屏 / 桌面 / 小组件 / 头像 / 锁屏文案）。

> MVP 版本：React Native + Expo + TypeScript，纯本地 mock 数据，不接后端、不接 AI、无需登录。
> 壁纸用**低饱和渐变占位**，零图片素材即可直接运行；接 AI 生图时把数据里的 `*Image` 字段换成图片 URL 即可。

## 技术栈
- Expo (SDK 51) + React Native 0.74 + TypeScript
- React Navigation（Bottom Tabs + Native Stack）
- expo-linear-gradient（渐变壁纸占位）/ expo-blur（毛玻璃）
- AsyncStorage（收藏本地持久化）

## 快速开始
```bash
# 1. 安装依赖
npm install

# 2. 启动开发服务器
npm start

# 3. 运行
#   - 手机装 Expo Go App，扫描终端二维码
#   - 或按 i 开 iOS 模拟器 / 按 a 开 Android 模拟器 / 按 w 开网页
```

类型检查：`npm run tsc`

## 目录结构
```
mood-wallpaper-app/
├── App.tsx                      # 入口：Provider + 导航
├── app.json                     # Expo 配置
├── tsconfig.json
├── package.json
└── src/
    ├── types.ts                 # 数据模型 & 导航类型
    ├── theme/tokens.ts          # 设计 Token（颜色/间距/圆角/字号）
    ├── data/
    │   ├── moods.ts             # 8 大情绪入口
    │   └── themes.ts            # 14 套主题包 + 数据访问 helper
    ├── context/
    │   └── FavoritesContext.tsx # 收藏状态（含持久化）
    ├── navigation/index.tsx     # Tabs + Stack
    ├── components/
    │   ├── GradientBackground.tsx
    │   ├── GlassCard.tsx
    │   ├── MoodCard.tsx
    │   ├── ThemeCard.tsx
    │   ├── QuoteCard.tsx
    │   ├── PhonePreview.tsx
    │   ├── Tag.tsx
    │   ├── SectionHeader.tsx
    │   └── FavoriteButton.tsx
    └── screens/
        ├── HomeScreen.tsx       # 今日情绪选择
        ├── ThemeListScreen.tsx  # 主题列表（按情绪筛选）
        ├── ThemeDetailScreen.tsx# 主题详情
        ├── PreviewScreen.tsx    # 手机锁屏/桌面预览
        └── FavoritesScreen.tsx  # 收藏
```

## 页面流
首页选情绪 → 主题列表 → 主题详情 → 手机预览；任意主题卡可收藏 → 收藏页。

## 数据结构（Theme）
```ts
type Theme = {
  id: string;
  name: string;
  mood: string;          // 对应情绪入口 key
  slogan: string;
  description: string;
  colors: string[];      // 主色板，同时驱动渐变占位图
  tags: string[];
  lockScreenImage: Gradient;
  homeScreenImage: Gradient;
  widgetImage: Gradient;
  avatarImage: Gradient;
  quotes: string[];      // 锁屏文案 / 金句
};
```

## 后续规划（V2+）
接入 AI 生图、一键应用到系统壁纸、情绪日历、分享卡片导出、社区与 UGC 主题。
