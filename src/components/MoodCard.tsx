// 情绪入口卡 —— 首页 8 大入口的网格卡片。
import React from 'react';
import { Pressable, Text, StyleSheet, View, StyleProp, ViewStyle } from 'react-native';
import GradientBackground from './GradientBackground';
import { palette, radius, spacing, typography, shadow } from '../theme/tokens';
import { Mood } from '../types';

type Props = {
  mood: Mood;
  onPress?: (mood: Mood) => void;
  style?: StyleProp<ViewStyle>;
};

export default function MoodCard({ mood, onPress, style }: Props) {
  return (
    <Pressable
      onPress={() => onPress?.(mood)}
      style={({ pressed }) => [
        styles.wrap,
        shadow.soft,
        style,
        pressed && { transform: [{ scale: 0.97 }], opacity: 0.95 },
      ]}
    >
      <GradientBackground colors={mood.colors} borderRadius={radius.lg} style={styles.bg}>
        <Text style={styles.emoji}>{mood.emoji}</Text>
        <View>
          <Text style={styles.label}>{mood.label}</Text>
          <Text style={styles.sub}>{mood.sub}</Text>
        </View>
      </GradientBackground>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  wrap: { borderRadius: radius.lg },
  bg: {
    height: 132,
    padding: spacing.lg,
    justifyContent: 'space-between',
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
  emoji: { fontSize: 26 },
  label: { ...typography.headline, color: palette.white },
  sub: { ...typography.micro, color: 'rgba(255,255,255,0.72)', marginTop: 4 },
});
