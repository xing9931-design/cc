// App 入口 —— 暗色主题 + 安全区 + 导航 + 收藏 Provider。
import 'react-native-gesture-handler';
import React from 'react';
import { StatusBar } from 'expo-status-bar';
import { SafeAreaProvider } from 'react-native-safe-area-context';
import { NavigationContainer, DefaultTheme, Theme as NavTheme } from '@react-navigation/native';

import RootNavigator from './src/navigation';
import { FavoritesProvider } from './src/context/FavoritesContext';
import { palette } from './src/theme/tokens';

// 导航容器用深色底，避免切页白闪
const navTheme: NavTheme = {
  ...DefaultTheme,
  dark: true,
  colors: {
    ...DefaultTheme.colors,
    background: palette.bg,
    card: palette.bg,
    text: palette.textPrimary,
    border: palette.border,
    primary: palette.accent,
  },
};

export default function App() {
  return (
    <SafeAreaProvider>
      <FavoritesProvider>
        <NavigationContainer theme={navTheme}>
          <StatusBar style="light" />
          <RootNavigator />
        </NavigationContainer>
      </FavoritesProvider>
    </SafeAreaProvider>
  );
}
