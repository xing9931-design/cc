// 导航 —— 底部 Tab（首页 / 收藏）外层包一个 Stack（主题列表 / 详情 / 预览）。
import React from 'react';
import { Platform } from 'react-native';
import { createNativeStackNavigator } from '@react-navigation/native-stack';
import { createBottomTabNavigator } from '@react-navigation/bottom-tabs';
import { BlurView } from 'expo-blur';
import { Ionicons } from '@expo/vector-icons';

import HomeScreen from '../screens/HomeScreen';
import FavoritesScreen from '../screens/FavoritesScreen';
import ThemeListScreen from '../screens/ThemeListScreen';
import ThemeDetailScreen from '../screens/ThemeDetailScreen';
import PreviewScreen from '../screens/PreviewScreen';
import { palette } from '../theme/tokens';
import { RootStackParamList, TabParamList } from '../types';

const Stack = createNativeStackNavigator<RootStackParamList>();
const Tab = createBottomTabNavigator<TabParamList>();

function Tabs() {
  return (
    <Tab.Navigator
      screenOptions={({ route }) => ({
        headerShown: false,
        tabBarActiveTintColor: palette.textPrimary,
        tabBarInactiveTintColor: palette.textTertiary,
        tabBarStyle: {
          position: 'absolute',
          borderTopWidth: 0,
          backgroundColor: Platform.OS === 'android' ? palette.bgElevated : 'transparent',
          elevation: 0,
          height: 84,
          paddingTop: 8,
        },
        tabBarBackground: () =>
          Platform.OS === 'ios' ? (
            <BlurView intensity={30} tint="dark" style={{ flex: 1 }} />
          ) : null,
        tabBarLabelStyle: { fontSize: 11, marginTop: 2 },
        tabBarIcon: ({ color, size, focused }) => {
          const icons: Record<keyof TabParamList, keyof typeof Ionicons.glyphMap> = {
            Home: focused ? 'sparkles' : 'sparkles-outline',
            Favorites: focused ? 'heart' : 'heart-outline',
          };
          return <Ionicons name={icons[route.name]} size={size - 2} color={color} />;
        },
      })}
    >
      <Tab.Screen name="Home" component={HomeScreen} options={{ title: '今日' }} />
      <Tab.Screen name="Favorites" component={FavoritesScreen} options={{ title: '收藏' }} />
    </Tab.Navigator>
  );
}

export default function RootNavigator() {
  return (
    <Stack.Navigator
      screenOptions={{
        headerShown: false,
        contentStyle: { backgroundColor: palette.bg },
        animation: 'slide_from_right',
      }}
    >
      <Stack.Screen name="Tabs" component={Tabs} />
      <Stack.Screen name="ThemeList" component={ThemeListScreen} />
      <Stack.Screen name="ThemeDetail" component={ThemeDetailScreen} />
      <Stack.Screen
        name="Preview"
        component={PreviewScreen}
        options={{ presentation: 'modal', animation: 'slide_from_bottom' }}
      />
    </Stack.Navigator>
  );
}
