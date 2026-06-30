// 主题列表页 —— 情绪筛选 Chips + 主题包列表（单列大卡）。
// 接收路由参数 moodKey（从首页情绪入口跳来）做默认筛选。
import React, { useState, useMemo, useEffect } from 'react';
import { View, Text, StyleSheet, FlatList, Pressable } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

import ThemeCard from '../components/ThemeCard';
import { MOODS, moodByKey } from '../data/moods';
import { getAllThemes, getThemesByMood } from '../data/themes';
import { palette, radius, spacing, typography } from '../theme/tokens';
import { ThemeListScreenProps, Mood } from '../types';

const ALL = { key: 'all', label: '全部', emoji: '' } as Pick<Mood, 'key' | 'label' | 'emoji'>;

function FilterChip({
  item,
  active,
  onPress,
}: {
  item: Pick<Mood, 'key' | 'label' | 'emoji'>;
  active: boolean;
  onPress: (key: string) => void;
}) {
  return (
    <Pressable
      onPress={() => onPress(item.key)}
      style={[styles.chip, active && styles.chipActive]}
    >
      <Text style={[styles.chipText, active && styles.chipTextActive]}>
        {item.emoji ? `${item.emoji} ` : ''}
        {item.label}
      </Text>
    </Pressable>
  );
}

export default function ThemeListScreen({ navigation, route }: ThemeListScreenProps) {
  const insets = useSafeAreaInsets();
  const [selected, setSelected] = useState<string>(route.params?.moodKey || 'all');

  useEffect(() => {
    if (route.params?.moodKey) setSelected(route.params.moodKey);
  }, [route.params?.moodKey]);

  const data = useMemo(
    () => (selected === 'all' ? getAllThemes() : getThemesByMood(selected)),
    [selected]
  );

  const chips = [ALL, ...MOODS];
  const title = selected === 'all' ? '发现主题' : moodByKey[selected]?.label ?? '发现主题';

  return (
    <View style={[styles.container, { paddingTop: insets.top + spacing.sm }]}>
      {/* 顶部返回 + 标题 */}
      <View style={styles.topBar}>
        <Pressable onPress={() => navigation.goBack()} hitSlop={10} style={styles.backBtn}>
          <Ionicons name="chevron-back" size={22} color={palette.textPrimary} />
        </Pressable>
        <Text style={styles.title}>{title}</Text>
      </View>

      {/* 筛选条 */}
      <FlatList
        horizontal
        data={chips}
        keyExtractor={(c) => c.key}
        showsHorizontalScrollIndicator={false}
        contentContainerStyle={styles.chipRow}
        style={styles.chipList}
        renderItem={({ item }) => (
          <FilterChip item={item} active={selected === item.key} onPress={setSelected} />
        )}
      />

      {/* 主题列表 */}
      <FlatList
        data={data}
        keyExtractor={(t) => t.id}
        showsVerticalScrollIndicator={false}
        contentContainerStyle={{ padding: spacing.lg, paddingBottom: 120, gap: spacing.lg }}
        renderItem={({ item }) => (
          <ThemeCard
            theme={item}
            onPress={(t) => navigation.navigate('ThemeDetail', { id: t.id })}
          />
        )}
        ListEmptyComponent={<Text style={styles.empty}>这个情绪还没有主题，换一个试试</Text>}
      />
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: palette.bg },
  topBar: {
    flexDirection: 'row',
    alignItems: 'center',
    gap: spacing.sm,
    paddingHorizontal: spacing.lg,
    marginBottom: spacing.md,
  },
  backBtn: { marginLeft: -6 },
  title: { ...typography.title, color: palette.textPrimary },
  chipList: { flexGrow: 0 },
  chipRow: { paddingHorizontal: spacing.lg, gap: spacing.sm, paddingBottom: spacing.md },
  chip: {
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.sm + 2,
    borderRadius: radius.pill,
    backgroundColor: palette.surface,
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  chipActive: { backgroundColor: palette.textPrimary, borderColor: palette.textPrimary },
  chipText: { ...typography.caption, color: palette.textSecondary },
  chipTextActive: { color: palette.bg, fontWeight: '600' },
  empty: {
    ...typography.body,
    color: palette.textTertiary,
    textAlign: 'center',
    marginTop: spacing.xxxl,
  },
});
