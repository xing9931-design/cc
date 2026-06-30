// 毛玻璃卡片 —— iOS 质感的关键。BlurView + 细边框。
import React, { ReactNode } from 'react';
import { View, StyleSheet, Platform, StyleProp, ViewStyle } from 'react-native';
import { BlurView } from 'expo-blur';
import { palette, radius, spacing } from '../theme/tokens';

type Props = {
  children: ReactNode;
  style?: StyleProp<ViewStyle>;
  intensity?: number;
  padding?: number;
};

export default function GlassCard({
  children,
  style,
  intensity = 24,
  padding = spacing.lg,
}: Props) {
  return (
    <View style={[styles.wrap, style]}>
      <BlurView
        intensity={intensity}
        tint="dark"
        style={[StyleSheet.absoluteFill, { borderRadius: radius.lg }]}
      />
      <View style={[styles.inner, { padding }]}>{children}</View>
    </View>
  );
}

const styles = StyleSheet.create({
  wrap: {
    borderRadius: radius.lg,
    overflow: 'hidden',
    backgroundColor: Platform.OS === 'android' ? palette.bgElevated : palette.surface,
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  inner: {
    backgroundColor: palette.surface,
  },
});
