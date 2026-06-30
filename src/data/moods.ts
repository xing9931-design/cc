// 8 大情绪入口 —— 首页强情绪入口配置。
// key 与 themes.ts 中的 mood 字段对应；colors 用于入口卡渐变占位。
import { Mood } from '../types';

export const MOODS: Mood[] = [
  {
    key: 'disappear',
    label: '今天想消失',
    emoji: '🫥',
    sub: '隐身一天，别找我',
    colors: ['#2B2D42', '#5C6378'],
  },
  {
    key: 'money',
    label: '今天只想搞钱',
    emoji: '🤑',
    sub: '闭嘴搞钱，少废话',
    colors: ['#1E2A1F', '#3E6B43'],
  },
  {
    key: 'noreply',
    label: '今天不想回消息',
    emoji: '📵',
    sub: '已读不回，理直气壮',
    colors: ['#26222E', '#4B3A5A'],
  },
  {
    key: 'reborn',
    label: '今天想重启人生',
    emoji: '🌱',
    sub: '格式化，重新开机',
    colors: ['#16242B', '#2E6E78'],
  },
  {
    key: 'crawl',
    label: '今天适合阴暗爬行',
    emoji: '🕷️',
    sub: '别管我，让我爬',
    colors: ['#151217', '#3A2A3F'],
  },
  {
    key: 'lowbattery',
    label: '今天低电量',
    emoji: '🪫',
    sub: '电量 1%，请勿打扰',
    colors: ['#241F16', '#6E5A2E'],
  },
  {
    key: 'coldface',
    label: '今天冷脸上岸',
    emoji: '🧊',
    sub: '清醒、冷淡、不解释',
    colors: ['#161E26', '#3A586E'],
  },
  {
    key: 'dopamine',
    label: '今天多巴胺复活',
    emoji: '🌈',
    sub: '快乐是会传染的',
    colors: ['#2B1626', '#7A3E63'],
  },
];

export const moodByKey: Record<string, Mood> = MOODS.reduce((acc, m) => {
  acc[m.key] = m;
  return acc;
}, {} as Record<string, Mood>);
