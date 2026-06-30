// 锁屏短句卡 —— 毛玻璃底，可直接当朋友圈/小红书文案。
import React from 'react';
import { Text, StyleSheet, StyleProp, ViewStyle } from 'react-native';
import GlassCard from './GlassCard';
import { palette, spacing, typography } from '../theme/tokens';

type Props = {
  quote: string;
  index?: number;
  style?: StyleProp<ViewStyle>;
};

export default function QuoteCard({ quote, index, style }: Props) {
  return (
    <GlassCard style={[styles.card, style]} padding={spacing.lg}>
      {typeof index === 'number' ? (
        <Text style={styles.index}>0{index + 1}</Text>
      ) : null}
      <Text style={styles.text}>“{quote}”</Text>
    </GlassCard>
  );
}

const styles = StyleSheet.create({
  card: { marginBottom: spacing.md },
  index: { ...typography.micro, color: palette.accent, marginBottom: spacing.xs },
  text: { ...typography.headline, color: palette.textPrimary, lineHeight: 26 },
});
