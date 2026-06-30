// 章节标题 —— 主标题 + 可选副标题 + 可选右侧操作。
import React from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { palette, spacing, typography } from '../theme/tokens';

type Props = {
  title: string;
  subtitle?: string;
  actionLabel?: string;
  onAction?: () => void;
};

export default function SectionHeader({ title, subtitle, actionLabel, onAction }: Props) {
  return (
    <View style={styles.row}>
      <View style={{ flex: 1 }}>
        <Text style={styles.title}>{title}</Text>
        {subtitle ? <Text style={styles.subtitle}>{subtitle}</Text> : null}
      </View>
      {actionLabel ? (
        <Pressable onPress={onAction} hitSlop={8}>
          <Text style={styles.action}>{actionLabel}</Text>
        </Pressable>
      ) : null}
    </View>
  );
}

const styles = StyleSheet.create({
  row: {
    flexDirection: 'row',
    alignItems: 'flex-end',
    marginBottom: spacing.lg,
  },
  title: { ...typography.headline, color: palette.textPrimary },
  subtitle: { ...typography.caption, color: palette.textTertiary, marginTop: 2 },
  action: { ...typography.caption, color: palette.accent },
});
