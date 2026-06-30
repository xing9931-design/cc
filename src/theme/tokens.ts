// 设计系统 Design Tokens —— 克制、高级、低饱和、iOS 质感。
// 统一管理颜色 / 间距 / 圆角 / 字号 / 阴影，避免硬编码散落各处。
import { TextStyle, ViewStyle } from 'react-native';

export const palette = {
  // 中性深色底（高级感来自深底 + 大留白）
  bg: '#0B0B0F',
  bgElevated: '#15151C',
  surface: 'rgba(255,255,255,0.06)',
  surfaceStrong: 'rgba(255,255,255,0.10)',
  border: 'rgba(255,255,255,0.12)',
  divider: 'rgba(255,255,255,0.08)',

  // 文本
  textPrimary: '#F5F5F7',
  textSecondary: 'rgba(245,245,247,0.62)',
  textTertiary: 'rgba(245,245,247,0.38)',

  // 情绪点缀色（低饱和）
  accent: '#A7A3FF',
  accentSoft: 'rgba(167,163,255,0.16)',
  danger: '#FF8E8E',
  white: '#FFFFFF',
  black: '#000000',
} as const;

export const spacing = {
  xs: 4,
  sm: 8,
  md: 12,
  lg: 16,
  xl: 24,
  xxl: 32,
  xxxl: 48,
} as const;

export const radius = {
  sm: 10,
  md: 16,
  lg: 22,
  xl: 28,
  pill: 999,
} as const;

export const typography: Record<string, TextStyle> = {
  display: { fontSize: 34, fontWeight: '700', letterSpacing: 0.2 },
  title: { fontSize: 24, fontWeight: '700', letterSpacing: 0.2 },
  headline: { fontSize: 19, fontWeight: '600' },
  body: { fontSize: 15, fontWeight: '400' },
  caption: { fontSize: 13, fontWeight: '400' },
  micro: { fontSize: 11, fontWeight: '500', letterSpacing: 0.4 },
};

export const shadow: Record<string, ViewStyle> = {
  // 克制的柔和阴影，不炸裂
  soft: {
    shadowColor: '#000',
    shadowOpacity: 0.35,
    shadowRadius: 24,
    shadowOffset: { width: 0, height: 12 },
    elevation: 8,
  },
};
