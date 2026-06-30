// 渐变占位图 —— MVP 用低饱和渐变替代真实壁纸素材。
// 接受 themes.ts 里的 Gradient（{colors,start,end}）或直接传 colors 数组。
import React, { ReactNode } from 'react';
import { StyleProp, ViewStyle } from 'react-native';
import { LinearGradient } from 'expo-linear-gradient';
import { radius } from '../theme/tokens';
import { Gradient } from '../types';

type Props = {
  image?: Gradient;
  colors?: string[];
  borderRadius?: number;
  style?: StyleProp<ViewStyle>;
  children?: ReactNode;
};

export default function GradientBackground({
  image,
  colors,
  borderRadius = radius.lg,
  style,
  children,
}: Props) {
  const stops = colors || image?.colors || ['#23232B', '#43434F'];
  const start = image?.start ? { x: image.start[0], y: image.start[1] } : { x: 0, y: 0 };
  const end = image?.end ? { x: image.end[0], y: image.end[1] } : { x: 1, y: 1 };

  // LinearGradient 要求至少 2 个色标
  const safeStops = stops.length > 1 ? stops : [stops[0], stops[0]];

  return (
    <LinearGradient
      colors={safeStops}
      start={start}
      end={end}
      style={[{ borderRadius }, style]}
    >
      {children}
    </LinearGradient>
  );
}
