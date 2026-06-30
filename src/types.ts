// 全局类型定义 —— 数据模型与导航参数。

/** 渐变占位图：MVP 用低饱和渐变替代真实壁纸素材 */
export type Gradient = {
  colors: string[];
  /** 渐变起点，[x, y]，取值 0~1 */
  start?: [number, number];
  /** 渐变终点，[x, y]，取值 0~1 */
  end?: [number, number];
};

/** 情绪入口 */
export type Mood = {
  key: string;
  label: string;
  emoji: string;
  sub: string;
  colors: string[];
};

/** 主题包 */
export type Theme = {
  id: string;
  name: string;
  mood: string;
  slogan: string;
  description: string;
  colors: string[];
  tags: string[];
  lockScreenImage: Gradient;
  homeScreenImage: Gradient;
  widgetImage: Gradient;
  avatarImage: Gradient;
  quotes: string[];
};

/** 根 Stack 路由参数表 */
export type RootStackParamList = {
  Tabs: undefined;
  ThemeList: { moodKey?: string } | undefined;
  ThemeDetail: { id: string };
  Preview: { id: string };
};

/** 底部 Tab 路由参数表 */
export type TabParamList = {
  Home: undefined;
  Favorites: undefined;
};

// —— 各页面 props 类型（给 screens 用）——
import type { NativeStackScreenProps } from '@react-navigation/native-stack';
import type { BottomTabScreenProps } from '@react-navigation/bottom-tabs';
import type { CompositeScreenProps } from '@react-navigation/native';

/** Tab 内页面：既能在 Tab 间切，也能 push 到 Stack 页 */
export type HomeScreenProps = CompositeScreenProps<
  BottomTabScreenProps<TabParamList, 'Home'>,
  NativeStackScreenProps<RootStackParamList>
>;
export type FavoritesScreenProps = CompositeScreenProps<
  BottomTabScreenProps<TabParamList, 'Favorites'>,
  NativeStackScreenProps<RootStackParamList>
>;

export type ThemeListScreenProps = NativeStackScreenProps<RootStackParamList, 'ThemeList'>;
export type ThemeDetailScreenProps = NativeStackScreenProps<RootStackParamList, 'ThemeDetail'>;
export type PreviewScreenProps = NativeStackScreenProps<RootStackParamList, 'Preview'>;
