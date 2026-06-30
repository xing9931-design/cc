// 标签胶囊 + 色板色块。两个轻量原子组件。
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { palette, radius, spacing, typography } from '../theme/tokens';

export function Tag({ label }: { label: string }) {
  return (
    <View style={styles.tag}>
      <Text style={styles.tagText}>{label}</Text>
    </View>
  );
}

export function ColorSwatch({ color, size = 28 }: { color: string; size?: number }) {
  return (
    <View
      style={[
        styles.swatch,
        { width: size, height: size, borderRadius: size / 3, backgroundColor: color },
      ]}
    />
  );
}

export default Tag;

const styles = StyleSheet.create({
  tag: {
    paddingHorizontal: spacing.md,
    paddingVertical: spacing.xs + 2,
    borderRadius: radius.pill,
    backgroundColor: palette.surfaceStrong,
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  tagText: {
    ...typography.caption,
    color: palette.textSecondary,
  },
  swatch: {
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
});
