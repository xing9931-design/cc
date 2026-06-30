// 收藏页 —— 展示收藏过的主题；无收藏时给一个克制的空状态。
import React, { useMemo } from 'react';
import { View, Text, StyleSheet, FlatList } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

import ThemeCard from '../components/ThemeCard';
import { getThemeById } from '../data/themes';
import { useFavorites } from '../context/FavoritesContext';
import { palette, spacing, typography } from '../theme/tokens';
import { FavoritesScreenProps, Theme } from '../types';

export default function FavoritesScreen({ navigation }: FavoritesScreenProps) {
  const insets = useSafeAreaInsets();
  const { favorites } = useFavorites();

  // 把收藏的 id 还原成主题对象（过滤掉可能已不存在的 id）
  const data = useMemo(
    () => favorites.map(getThemeById).filter((t): t is Theme => Boolean(t)),
    [favorites]
  );

  return (
    <View style={[styles.container, { paddingTop: insets.top + spacing.lg }]}>
      <View style={styles.header}>
        <Text style={styles.title}>我的收藏</Text>
        <Text style={styles.count}>{data.length} 套主题</Text>
      </View>

      {data.length === 0 ? (
        <View style={styles.emptyWrap}>
          <Ionicons name="heart-outline" size={44} color={palette.textTertiary} />
          <Text style={styles.emptyTitle}>还没有收藏</Text>
          <Text style={styles.emptySub}>遇到对味的主题，点右上角的心收藏起来</Text>
        </View>
      ) : (
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
        />
      )}
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: palette.bg },
  header: { paddingHorizontal: spacing.lg, marginBottom: spacing.lg },
  title: { ...typography.display, color: palette.textPrimary },
  count: { ...typography.caption, color: palette.textTertiary, marginTop: spacing.xs },
  emptyWrap: { flex: 1, alignItems: 'center', justifyContent: 'center', gap: spacing.sm, paddingBottom: 80 },
  emptyTitle: { ...typography.headline, color: palette.textSecondary, marginTop: spacing.sm },
  emptySub: { ...typography.caption, color: palette.textTertiary, textAlign: 'center', paddingHorizontal: spacing.xxl },
});
