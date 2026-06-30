// 首页 —— 今日情绪选择：App 名 + 副标题 → 8 大情绪入口 → 今日推荐横滑。
import React from 'react';
import { View, Text, StyleSheet, ScrollView, FlatList } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';

import MoodCard from '../components/MoodCard';
import ThemeCard from '../components/ThemeCard';
import SectionHeader from '../components/SectionHeader';
import { MOODS } from '../data/moods';
import { getDailyPicks } from '../data/themes';
import { palette, spacing, typography } from '../theme/tokens';
import { HomeScreenProps, Mood, Theme } from '../types';

export default function HomeScreen({ navigation }: HomeScreenProps) {
  const insets = useSafeAreaInsets();
  const picks = getDailyPicks(6);

  const openMood = (mood: Mood) => navigation.navigate('ThemeList', { moodKey: mood.key });
  const openTheme = (theme: Theme) => navigation.navigate('ThemeDetail', { id: theme.id });

  return (
    <ScrollView
      style={styles.container}
      contentContainerStyle={{ paddingTop: insets.top + spacing.lg, paddingBottom: 120 }}
      showsVerticalScrollIndicator={false}
    >
      {/* 顶部：App 名 + 副标题 */}
      <View style={styles.header}>
        <Text style={styles.kicker}>PINGXU</Text>
        <Text style={styles.appName}>屏绪</Text>
        <Text style={styles.subtitle}>今天你的手机是什么情绪？</Text>
      </View>

      {/* 情绪入口网格 */}
      <View style={styles.grid}>
        {MOODS.map((mood) => (
          <MoodCard key={mood.key} mood={mood} onPress={openMood} style={styles.gridItem} />
        ))}
      </View>

      {/* 今日推荐 */}
      <View style={styles.section}>
        <View style={{ paddingRight: spacing.lg }}>
          <SectionHeader
            title="今日推荐"
            subtitle="为今天的你挑了几套"
            actionLabel="全部 ›"
            onAction={() => navigation.navigate('ThemeList')}
          />
        </View>
        <FlatList
          horizontal
          data={picks}
          keyExtractor={(t) => t.id}
          showsHorizontalScrollIndicator={false}
          contentContainerStyle={{ paddingRight: spacing.lg }}
          renderItem={({ item }) => (
            <ThemeCard theme={item} variant="rail" onPress={openTheme} />
          )}
        />
      </View>
    </ScrollView>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: palette.bg },
  header: { paddingHorizontal: spacing.lg, marginBottom: spacing.xl },
  kicker: { ...typography.micro, color: palette.accent, marginBottom: spacing.xs },
  appName: { fontSize: 42, fontWeight: '800', color: palette.textPrimary, letterSpacing: 2 },
  subtitle: { ...typography.body, color: palette.textSecondary, marginTop: spacing.sm },
  grid: {
    paddingHorizontal: spacing.lg,
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
  },
  gridItem: { width: '48.5%', marginBottom: spacing.md },
  section: { marginTop: spacing.xl, paddingLeft: spacing.lg },
});
