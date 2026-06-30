// 收藏按钮 —— 连 FavoritesContext，半透明圆形底。
import React from 'react';
import { Pressable, StyleSheet, View } from 'react-native';
import { Ionicons } from '@expo/vector-icons';
import { palette } from '../theme/tokens';
import { useFavorites } from '../context/FavoritesContext';

export default function FavoriteButton({
  themeId,
  size = 22,
}: {
  themeId: string;
  size?: number;
}) {
  const { isFavorite, toggleFavorite } = useFavorites();
  const active = isFavorite(themeId);

  return (
    <Pressable
      hitSlop={10}
      onPress={() => toggleFavorite(themeId)}
      style={({ pressed }) => [styles.btn, pressed && { transform: [{ scale: 0.9 }] }]}
    >
      <View style={styles.bg}>
        <Ionicons
          name={active ? 'heart' : 'heart-outline'}
          size={size}
          color={active ? palette.danger : palette.white}
        />
      </View>
    </Pressable>
  );
}

const styles = StyleSheet.create({
  btn: { borderRadius: 999 },
  bg: {
    width: 38,
    height: 38,
    borderRadius: 19,
    alignItems: 'center',
    justifyContent: 'center',
    backgroundColor: 'rgba(0,0,0,0.35)',
    borderWidth: StyleSheet.hairlineWidth,
    borderColor: palette.border,
  },
});
