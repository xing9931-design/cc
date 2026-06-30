// 主题详情页 —— 主视觉 → 标题/slogan/描述 → 色板 → 内容清单 → 锁屏短句 → CTA。
import React from 'react';
import { View, Text, StyleSheet, ScrollView, Pressable } from 'react-native';
import { useSafeAreaInsets } from 'react-native-safe-area-context';
import { Ionicons } from '@expo/vector-icons';

import GradientBackground from '../components/GradientBackground';
import QuoteCard from '../components/QuoteCard';
import FavoriteButton from '../components/FavoriteButton';
import SectionHeader from '../components/SectionHeader';
import { Tag, ColorSwatch } from '../components/Tag';
import { getThemeById } from '../data/themes';
import { moodByKey } from '../data/moods';
import { palette, radius, spacing, typography, shadow } from '../theme/tokens';
import { ThemeDetailScreenProps, Gradient } from '../types';

// 主题包内容清单四件套
const ASSETS = [
  { key: 'lockScreenImage', label: '锁屏壁纸', icon: 'lock-closed-outline' },
  { key: 'homeScreenImage', label: '桌面壁纸', icon: 'phone-portrait-outline' },
  { key: 'widgetImage', label: '小组件背景', icon: 'grid-outline' },
  { key: 'avatarImage', label: '头像图', icon: 'person-circle-outline' },
] as const;

function AssetTile({
  image,
  label,
  icon,
}: {
  image: Gradient;
  label: string;
  icon: keyof typeof Ionicons.glyphMap;
}) {
  return (
    <View style={styles.assetTile}>
      <GradientBackground image={image} borderRadius={radius.md} style={styles.assetThumb} />
      <View style={styles.assetMeta}>
        <Ionicons name={icon} size={15} color={palette.textSecondary} />
        <Text style={styles.assetLabel}>{label}</Text>
      </View>
    </View>
  );
}

export default function ThemeDetailScreen({ navigation, route }: ThemeDetailScreenProps) {
  const insets = useSafeAreaInsets();
  const theme = getThemeById(route.params.id);

  if (!theme) {
    return (
      <View style={[styles.container, styles.center]}>
        <Text style={styles.empty}>主题不存在</Text>
      </View>
    );
  }

  const mood = moodByKey[theme.mood];

  return (
    <View style={styles.container}>
      <ScrollView
        showsVerticalScrollIndicator={false}
        contentContainerStyle={{ paddingBottom: 120 }}
      >
        {/* 主视觉 */}
        <GradientBackground image={theme.lockScreenImage} borderRadius={0} style={styles.hero}>
          <View style={[styles.heroTop, { paddingTop: insets.top + spacing.sm }]}>
            <Pressable onPress={() => navigation.goBack()} hitSlop={10} style={styles.iconBtn}>
              <Ionicons name="chevron-back" size={22} color={palette.white} />
            </Pressable>
            <FavoriteButton themeId={theme.id} />
          </View>

          <View>
            {mood ? (
              <View style={styles.moodPill}>
                <Text style={styles.moodPillText}>
                  {mood.emoji} {mood.label}
                </Text>
              </View>
            ) : null}
            <Text style={styles.heroName}>{theme.name}</Text>
            <Text style={styles.heroSlogan}>{theme.slogan}</Text>
          </View>
        </GradientBackground>

        <View style={styles.body}>
          {/* 描述 */}
          <Text style={styles.desc}>{theme.description}</Text>

          {/* 标签 */}
          <View style={styles.tagRow}>
            {theme.tags.map((t) => (
              <Tag key={t} label={`#${t}`} />
            ))}
          </View>

          {/* 色板 */}
          <View style={styles.block}>
            <SectionHeader title="情绪色板" subtitle="驱动整套主题的主色" />
            <View style={styles.swatchRow}>
              {theme.colors.map((c) => (
                <View key={c} style={styles.swatchItem}>
                  <ColorSwatch color={c} size={40} />
                  <Text style={styles.swatchHex}>{c}</Text>
                </View>
              ))}
            </View>
          </View>

          {/* 内容清单 */}
          <View style={styles.block}>
            <SectionHeader title="主题内容" subtitle="一套配齐，开箱即用" />
            <View style={styles.assetGrid}>
              {ASSETS.map((a) => (
                <AssetTile key={a.key} image={theme[a.key]} label={a.label} icon={a.icon} />
              ))}
            </View>
          </View>

          {/* 锁屏短句 */}
          <View style={styles.block}>
            <SectionHeader title="锁屏文案" subtitle="也能直接发朋友圈 / 小红书" />
            {theme.quotes.map((q, i) => (
              <QuoteCard key={i} quote={q} index={i} />
            ))}
          </View>
        </View>
      </ScrollView>

      {/* 底部 CTA */}
      <View style={[styles.ctaBar, { paddingBottom: insets.bottom + spacing.md }]}>
        <Pressable
          style={({ pressed }) => [styles.cta, shadow.soft, pressed && { opacity: 0.9 }]}
          onPress={() => navigation.navigate('Preview', { id: theme.id })}
        >
          <Ionicons name="phone-portrait-outline" size={18} color={palette.bg} />
          <Text style={styles.ctaText}>预览手机效果</Text>
        </Pressable>
      </View>
    </View>
  );
}

const styles = StyleSheet.create({
  container: { flex: 1, backgroundColor: palette.bg },
  center: { alignItems: 'center', justifyContent: 'center' },
  empty: { ...typography.body, color: palette.textTertiary },

  hero: { height: 420, padding: spacing.lg, justifyContent: 'space-between' },
  heroTop: { flexDirection: 'row', justifyContent: 'space-between', alignItems: 'center' },
  iconBtn: {
    width: 38,
    height: 38,
    borderRadius: 19,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'rgba(0,0,0,0.35)',
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  moodPill: {
    alignSelf: 'flex-start',
    paddingHorizontal: spacing.md,
    paddingVertical: 6,
    borderRadius: radius.pill,
    backgroundColor: 'rgba(0,0,0,0.30)',
    marginBottom: spacing.md,
  },
  moodPillText: { ...typography.micro, color: 'rgba(255,255,255,0.92)' },
  heroName: { fontSize: 40, fontWeight: '800', color: palette.white, letterSpacing: 0.5 },
  heroSlogan: { ...typography.headline, color: 'rgba(255,255,255,0.82)', marginTop: 4 },

  body: { padding: spacing.lg },
  desc: { ...typography.body, color: palette.textSecondary, lineHeight: 24 },
  tagRow: { flexDirection: 'row', flexWrap: 'wrap', gap: spacing.sm, marginTop: spacing.lg },

  block: { marginTop: spacing.xxl },
  swatchRow: { flexDirection: 'row', gap: spacing.xl },
  swatchItem: { alignItems: 'center', gap: spacing.sm },
  swatchHex: { ...typography.micro, color: palette.textTertiary },

  assetGrid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
    rowGap: spacing.lg,
  },
  assetTile: { width: '48%' },
  assetThumb: { height: 110, borderWidth: StyleSheet.hairlineWidth, borderColor: palette.border },
  assetMeta: { flexDirection: 'row', alignItems: 'center', gap: 6, marginTop: spacing.sm },
  assetLabel: { ...typography.caption, color: palette.textSecondary },

  ctaBar: {
    position: 'absolute',
    bottom: 0,
    left: 0,
    right: 0,
    paddingHorizontal: spacing.lg,
    paddingTop: spacing.md,
    backgroundColor: 'rgba(11,11,15,0.6)',
  },
  cta: {
    flexDirection: 'row',
    alignItems: 'center',
    justifyContent: 'center',
    gap: spacing.sm,
    height: 54,
    borderRadius: radius.pill,
    backgroundColor: palette.textPrimary,
  },
  ctaText: { ...typography.headline, color: palette.bg },
});
