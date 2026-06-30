// 手机模拟器 —— 预览页用，渲染锁屏 / 桌面两种状态。
import React from 'react';
import { View, Text, StyleSheet } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import GradientBackground from './GradientBackground';
import { palette, radius, spacing, typography } from '../theme/tokens';
import { Theme } from '../types';

export type PreviewMode = 'lock' | 'home';

// 桌面 App 图标占位（用色板循环上色）
function AppIcon({ color }: { color: string }) {
  return <View style={[styles.appIcon, { backgroundColor: color }]} />;
}

export default function PhonePreview({
  theme,
  mode = 'lock',
}: {
  theme: Theme;
  mode?: PreviewMode;
}) {
  const isLock = mode === 'lock';
  const image = isLock ? theme.lockScreenImage : theme.homeScreenImage;
  const colors = theme.colors;

  return (
    <View style={styles.frame}>
      <View style={styles.notch} />
      <GradientBackground image={image} borderRadius={34} style={styles.screen}>
        {/* 状态栏 */}
        <View style={styles.statusBar}>
          <Text style={styles.statusText}>屏绪</Text>
          <View style={styles.statusIcons}>
            <Ionicons name="cellular" size={13} color="rgba(255,255,255,0.85)" />
            <Ionicons name="wifi" size={13} color="rgba(255,255,255,0.85)" />
            <Ionicons name="battery-half" size={15} color="rgba(255,255,255,0.85)" />
          </View>
        </View>

        {isLock ? (
          // —— 锁屏：时钟 + 情绪文案 ——
          <View style={styles.lockBody}>
            <Text style={styles.lockDate}>6月30日 星期二</Text>
            <Text style={styles.lockClock}>9:41</Text>
            <View style={styles.quoteCard}>
              <Text style={styles.quoteText}>{theme.quotes[0]}</Text>
            </View>
          </View>
        ) : (
          // —— 桌面：小组件 + App 网格 + Dock ——
          <View style={styles.homeBody}>
            <GradientBackground
              image={theme.widgetImage}
              borderRadius={radius.lg}
              style={styles.widget}
            >
              <Text style={styles.widgetTitle}>{theme.name}</Text>
              <Text style={styles.widgetQuote} numberOfLines={2}>
                {theme.quotes[1] || theme.slogan}
              </Text>
            </GradientBackground>

            <View style={styles.grid}>
              {Array.from({ length: 16 }).map((_, i) => (
                <AppIcon key={i} color={colors[i % colors.length]} />
              ))}
            </View>

            <View style={styles.dock}>
              {Array.from({ length: 4 }).map((_, i) => (
                <AppIcon key={i} color={colors[(i + 1) % colors.length]} />
              ))}
            </View>
          </View>
        )}
      </GradientBackground>
    </View>
  );
}

const styles = StyleSheet.create({
  frame: {
    width: 270,
    height: 560,
    borderRadius: 42,
    padding: 8,
    backgroundColor: '#000',
    borderWidth: 2,
    borderColor: 'rgba(255,255,255,0.10)',
    alignSelf: 'center',
  },
  notch: {
    position: 'absolute',
    top: 14,
    alignSelf: 'center',
    width: 100,
    height: 26,
    borderRadius: 14,
    backgroundColor: '#000',
    zIndex: 10,
  },
  screen: { flex: 1, overflow: 'hidden', paddingTop: 16 },
  statusBar: {
    flexDirection: 'row',
    justifyContent: 'space-between',
    alignItems: 'center',
    paddingHorizontal: spacing.lg,
    paddingTop: 6,
  },
  statusText: { ...typography.micro, color: 'rgba(255,255,255,0.85)' },
  statusIcons: { flexDirection: 'row', alignItems: 'center', gap: 4 },

  // 锁屏
  lockBody: { flex: 1, alignItems: 'center', paddingTop: 40 },
  lockDate: { ...typography.caption, color: 'rgba(255,255,255,0.80)' },
  lockClock: {
    fontSize: 76,
    fontWeight: '700',
    color: palette.white,
    letterSpacing: 1,
    marginTop: -4,
  },
  quoteCard: {
    marginTop: 'auto',
    marginBottom: 40,
    paddingHorizontal: spacing.lg,
    paddingVertical: spacing.md,
    borderRadius: radius.lg,
    backgroundColor: 'rgba(0,0,0,0.28)',
  },
  quoteText: { ...typography.body, color: palette.white, textAlign: 'center' },

  // 桌面
  homeBody: { flex: 1, paddingHorizontal: spacing.lg, paddingTop: spacing.lg },
  widget: { height: 96, padding: spacing.md, justifyContent: 'center', marginBottom: spacing.lg },
  widgetTitle: { ...typography.headline, color: palette.white },
  widgetQuote: { ...typography.caption, color: 'rgba(255,255,255,0.80)', marginTop: 4 },
  grid: {
    flexDirection: 'row',
    flexWrap: 'wrap',
    justifyContent: 'space-between',
    rowGap: spacing.lg,
  },
  appIcon: { width: 44, height: 44, borderRadius: 12 },
  dock: {
    marginTop: 'auto',
    marginBottom: spacing.lg,
    flexDirection: 'row',
    justifyContent: 'space-around',
    paddingVertical: spacing.md,
    borderRadius: radius.xl,
    backgroundColor: 'rgba(0,0,0,0.22)',
  },
});
