// 手机预览页 —— 模拟一台手机，可切换锁屏 / 桌面。
import React, { useState } from 'react';
import { View, Text, StyleSheet, Pressable } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

import PhonePreview, { PreviewMode } from '../components/PhonePreview';
import { getThemeById } from '../data/themes';
import { palette, radius, spacing, typography } from '../theme/tokens';
import { PreviewScreenProps } from '../types';

const SEGMENTS: { key: PreviewMode; label: string }[] = [
  { key: 'lock', label: '锁屏' },
  { key: 'home', label: '桌面' },
];

export default function PreviewScreen({ navigation, route }: PreviewScreenProps) {
  const insets = useSafeAreaInsets();
  const theme = getThemeById(route.params.id);
  const [mode, setMode] = useState<PreviewMode>('lock');

  if (!theme) {
    return (
      <View style={[styles.container, styles.center]}>
        <Text style={styles.empty}>主题不存在</Text>
      </View>
    );
  }

  return (
    <View style={[styles.container, { paddingTop: insets.top + spacing.sm }]}>
      {/* 顶部栏 */}
      <View style={styles.topBar}>
        <Pressable onPress={() => navigation.goBack()} hitSlop={10}>
          <Ionicons name="close" size={26} color={palette.textPrimary} />
        </Pressable>
        <Text style={styles.title}>{theme.name}</Text>
        <View style={{ width: 26 }} />
      </View>

      {/* 手机模拟 */}
      <View style={styles.stage}>
        <PhonePreview theme={theme} mode={mode} />
      </View>

      {/* 锁屏 / 桌面 切换 */}
      <View style={[styles.segmentWrap, { paddingBottom: insets.bottom + spacing.xl }]}>
        <View style={styles.segment}>
          {SEGMENTS.map((s) => {
            const active = mode === s.key;
            return (
              <Pressable
                key={s.key}
                onPress={() => setMode(s.key)}
                style={[styles.segmentItem, active && styles.segmentItemActive]}
              >
                <Text style={[styles.segmentText, active && styles.segmentTextActive]}>
                  {s.label}
                </Text>
              </Pressable>
            );
          })}
        </View>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: palette.bg },
  center: { alignItems: 'center', justifyContent: 'center' },
  empty: { ...typography.body, color: palette.textTertiary },
  topBar: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'space-between',
    paddingHorizontal: spacing.lg,
  },
  title: { ...typography.headline, color: palette.textPrimary },
  stage: { flex: 1, alignItems: 'center', justifyContent: 'center' },
  segmentWrap: { alignItems: 'center' },
  segment: {
    flexDirection: 'row',
    padding: 4,
    borderRadius: radius.pill,
    backgroundColor: palette.surface,
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  segmentItem: {
    paddingHorizontal: spacing.xxl,
    paddingVertical: spacing.sm + 2,
    borderRadius: radius.pill,
  },
  segmentItemActive: { backgroundColor: palette.textPrimary },
  segmentText: { ...typography.body, color: palette.textSecondary, fontWeight: '600' },
  segmentTextActive: { color: palette.bg },
});
