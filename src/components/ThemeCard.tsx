// 主题包卡片 —— 列表/推荐通用。封面渐变 + 名称 + slogan + 标签 + 收藏心。
// variant: 'grid'（单列大卡）| 'rail'（首页横滑小卡）
import React from 'react';
import { Pressable, Text, StyleSheet, View, StyleProp, ViewStyle } from 'react-native';
import GradientBackground from './GradientBackground';
import FavoriteButton from './FavoriteButton';
import { palette, radius, spacing, typography, shadow } from '../theme/tokens';
import { Theme } from '../types';

type Props = {
  theme: Theme;
  onPress?: (theme: Theme) => void;
  variant?: 'grid' | 'rail';
  style?: StyleProp<ViewStyle>;
};

export default function ThemeCard({ theme, onPress, variant = 'grid', style }: Props) {
  const isRail = variant === 'rail';
  return (
    <Pressable
      onPress={() => onPress?.(theme)}
      style={({ pressed }) => [
        isRail ? styles.railWrap : styles.gridWrap,
        shadow.soft,
        style,
        pressed && { transform: [{ scale: 0.98 }] },
      ]}
    >
      <GradientBackground
        image={theme.lockScreenImage}
        borderRadius={radius.lg}
        style={[styles.cover, { height: isRail ? 200 : 230 }]}
      >
        <View style={styles.topRow}>
          <View style={styles.sloganPill}>
            <Text style={styles.sloganText} numberOfLines={1}>
              {theme.slogan}
            </Text>
          </View>
          <FavoriteButton themeId={theme.id} />
        </View>

        <View>
          <Text style={styles.name}>{theme.name}</Text>
          {!isRail ? (
            <View style={styles.tagRow}>
              {theme.tags.slice(0, 3).map((t) => (
                <View key={t} style={styles.tag}>
                  <Text style={styles.tagText}>#{t}</Text>
                </View>
              ))}
            </View>
          ) : (
            <Text style={styles.quote} numberOfLines={1}>
              {theme.quotes[0]}
            </Text>
          )}
        </View>
      </GradientBackground>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  gridWrap: { borderRadius: radius.lg },
  railWrap: { width: 160, borderRadius: radius.lg, marginRight: spacing.md },
  cover: {
    padding: spacing.md,
    justifyContent: 'space-between',
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  topRow: {
    flexDirection: 'row',
    alignItems: 'flex-start',
    justifyContent: 'space-between',
  },
  sloganPill: {
    maxWidth: '72%',
    paddingHorizontal: spacing.sm + 2,
    paddingVertical: 5,
    borderRadius: radius.pill,
    backgroundColor: 'rgba(0,0,0,0.30)',
  },
  sloganText: { ...typography.micro, color: 'rgba(255,255,255,0.92)' },
  name: { ...typography.title, fontSize: 22, color: palette.white },
  quote: { ...typography.caption, color: 'rgba(255,255,255,0.78)', marginTop: 2 },
  tagRow: { flexDirection: 'row', gap: spacing.xs, marginTop: spacing.sm },
  tag: {
    paddingHorizontal: spacing.sm,
    paddingVertical: 3,
    borderRadius: radius.pill,
    backgroundColor: 'rgba(0,0,0,0.28)',
  },
  tagText: { ...typography.micro, color: 'rgba(255,255,255,0.85)' },
});
