// 主题包数据 —— 本地 mock「假后端」。未来换成 API/AI 生图只改这里。
// 每套主题的 *Image 字段在 MVP 用渐变占位（零图片素材即可运行）。
import { Theme } from '../types';

export const THEMES: Theme[] = [
  {
    id: 'disappear-01',
    name: '人间蒸发',
    mood: 'disappear',
    slogan: '我先走了，别找我',
    description:
      '一套适合「想消失」的低存在感主题。雾灰蓝调，把自己调成隐身模式，世界吵不到你。',
    colors: ['#2B2D42', '#5C6378', '#9AA0B5'],
    tags: ['冷淡', '隐身', '低存在感'],
    lockScreenImage: { colors: ['#1B1D2B', '#3A3F57'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#23263A', '#52596F'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#2B2D42', '#5C6378'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#5C6378', '#9AA0B5'], start: [0, 0], end: [1, 1] },
    quotes: ['查无此人', '已隐身，请勿打扰', '我在，但我不在'],
  },
  {
    id: 'disappear-02',
    name: '失踪人口',
    mood: 'disappear',
    slogan: '下次出现是惊喜',
    description: '雾面深空灰，留白克制，适合想从所有群聊里消失一阵子的你。',
    colors: ['#23232B', '#43434F', '#7C7C8A'],
    tags: ['留白', '深空灰', '断联'],
    lockScreenImage: { colors: ['#18181E', '#33333D'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#202028', '#3D3D49'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#23232B', '#43434F'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#43434F', '#7C7C8A'], start: [0, 0], end: [1, 1] },
    quotes: ['失踪了，挺好的', '别等我了', '下线，不解释'],
  },
  {
    id: 'money-01',
    name: '闭嘴搞钱',
    mood: 'money',
    slogan: '情绪稳定，账户增长',
    description:
      '克制的墨绿与暗金，去掉一切浮夸的暴富感，留下冷静的搞钱叙事。专注，沉默，复利。',
    colors: ['#1E2A1F', '#3E6B43', '#A9C7A0'],
    tags: ['搞钱', '墨绿', '清醒'],
    lockScreenImage: { colors: ['#121C13', '#2C4B30'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#16241A', '#345A3A'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#1E2A1F', '#3E6B43'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#3E6B43', '#A9C7A0'], start: [0, 0], end: [1, 1] },
    quotes: ['少说话，多搞钱', '情绪稳定是顶级搞钱能力', '复利在悄悄发生'],
  },
  {
    id: 'money-02',
    name: '暗金叙事',
    mood: 'money',
    slogan: '野心要低调',
    description: '近黑底加一缕暗金光，适合上岸路上闷声努力的你。高级，沉得住气。',
    colors: ['#1A1712', '#5A4A24', '#C8A85A'],
    tags: ['暗金', '野心', '低调'],
    lockScreenImage: { colors: ['#13110C', '#3F3318'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#19150E', '#4A3C1E'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#1A1712', '#5A4A24'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#5A4A24', '#C8A85A'], start: [0, 0], end: [1, 1] },
    quotes: ['闷声发大财', '野心藏在沉默里', '结果会替我说话'],
  },
  {
    id: 'noreply-01',
    name: '已读不回',
    mood: 'noreply',
    slogan: '看到了，不想回',
    description:
      '暗紫调，安静而有边界感。把消息红点关掉，给自己一个不被打扰的下午。',
    colors: ['#26222E', '#4B3A5A', '#9B86B0'],
    tags: ['边界感', '暗紫', '断联'],
    lockScreenImage: { colors: ['#1A1722', '#382B45'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#221E2B', '#43344F'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#26222E', '#4B3A5A'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#4B3A5A', '#9B86B0'], start: [0, 0], end: [1, 1] },
    quotes: ['已读，不回', '消息可以等，我不可以', '设边界不是冷漠'],
  },
  {
    id: 'noreply-02',
    name: '勿扰模式',
    mood: 'noreply',
    slogan: '世界静音中',
    description: '雾蓝灰，安静到能听见自己呼吸。今天把通知全关，谁也别想打扰。',
    colors: ['#1F2329', '#3D4654', '#8A95A6'],
    tags: ['静音', '雾蓝', '独处'],
    lockScreenImage: { colors: ['#16191F', '#2E3743'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#1B1F26', '#36414F'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#1F2329', '#3D4654'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#3D4654', '#8A95A6'], start: [0, 0], end: [1, 1] },
    quotes: ['世界已静音', '通知全关，人很安静', '今天不在服务区'],
  },
  {
    id: 'reborn-01',
    name: '重新开机',
    mood: 'reborn',
    slogan: '格式化，再来一次',
    description:
      '清透的青绿到雾绿，像清晨第一口空气。适合想把过去清空、重启人生的时刻。',
    colors: ['#16242B', '#2E6E78', '#9FD6CE'],
    tags: ['重启', '青绿', '清新'],
    lockScreenImage: { colors: ['#0F1A1F', '#235157'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#142026', '#2A626B'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#16242B', '#2E6E78'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#2E6E78', '#9FD6CE'], start: [0, 0], end: [1, 1] },
    quotes: ['重启人生进行中', '旧的就让它过去', '今天是第一天'],
  },
  {
    id: 'reborn-02',
    name: '新版本的我',
    mood: 'reborn',
    slogan: 'v2.0 已上线',
    description: '雾蓝到淡绿，干净通透。把自己当一次更新，删掉 bug，留下新功能。',
    colors: ['#1A2430', '#3C6E7A', '#BFE3DA'],
    tags: ['升级', '通透', '自救'],
    lockScreenImage: { colors: ['#121A22', '#2E545E'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#16202A', '#356068'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#1A2430', '#3C6E7A'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#3C6E7A', '#BFE3DA'], start: [0, 0], end: [1, 1] },
    quotes: ['新版本，已修复', '我在迭代，请稍候', '更稳定的我上线了'],
  },
  {
    id: 'crawl-01',
    name: '阴暗爬行',
    mood: 'crawl',
    slogan: '别管我，让我爬',
    description:
      '近黑的暗紫，深夜感拉满。适合不想被看见、只想在角落里安静崩溃又自愈的你。',
    colors: ['#151217', '#3A2A3F', '#6E5A73'],
    tags: ['发疯', '深夜', '暗黑'],
    lockScreenImage: { colors: ['#0D0B0F', '#2C2030'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#120F14', '#332639'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#151217', '#3A2A3F'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#3A2A3F', '#6E5A73'], start: [0, 0], end: [1, 1] },
    quotes: ['阴暗爬行中…', '别开灯，让我爬', '崩溃五分钟，继续做人'],
  },
  {
    id: 'lowbattery-01',
    name: '低电量模式',
    mood: 'lowbattery',
    slogan: '电量 1%，请勿打扰',
    description:
      '暖棕到暗金，像手机进入省电模式的那抹黄。今天只剩一点电，省着用。',
    colors: ['#241F16', '#6E5A2E', '#D6B96A'],
    tags: ['摆烂', '省电', '暖调'],
    lockScreenImage: { colors: ['#1A160F', '#4F4020'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#201B12', '#5C4A26'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#241F16', '#6E5A2E'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#6E5A2E', '#D6B96A'], start: [0, 0], end: [1, 1] },
    quotes: ['电量 1%', '已开启省电模式', '今天只够爱自己'],
  },
  {
    id: 'lowbattery-02',
    name: '请勿充电',
    mood: 'lowbattery',
    slogan: '没电了，别叫我',
    description: '低饱和暖灰，像关机前最后一格电。今天就摆烂，谁来都不充。',
    colors: ['#1F1C18', '#4A4338', '#9C8F78'],
    tags: ['摆烂', '暖灰', '关机'],
    lockScreenImage: { colors: ['#161412', '#3A352C'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#1B1814', '#433C32'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#1F1C18', '#4A4338'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#4A4338', '#9C8F78'], start: [0, 0], end: [1, 1] },
    quotes: ['没电了，别叫我', '今天不营业', '摆烂也是一种养生'],
  },
  {
    id: 'coldface-01',
    name: '冷脸上岸',
    mood: 'coldface',
    slogan: '清醒、冷淡、不解释',
    description:
      '冰川蓝灰，理性克制。适合谈判、面试、上岸前那种清醒到发冷的状态。',
    colors: ['#161E26', '#3A586E', '#A6C2D6'],
    tags: ['清醒', '冰蓝', '上岸'],
    lockScreenImage: { colors: ['#10161C', '#2C4252'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#141B22', '#345064'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#161E26', '#3A586E'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#3A586E', '#A6C2D6'], start: [0, 0], end: [1, 1] },
    quotes: ['冷静是我的保护色', '上岸前不动声色', '不解释，看结果'],
  },
  {
    id: 'dopamine-01',
    name: '多巴胺复活',
    mood: 'dopamine',
    slogan: '快乐是会传染的',
    description:
      '低饱和多巴胺，莓粉到暖橘，明亮但不刺眼。适合想从低气压里重新开机的好日子。',
    colors: ['#2B1626', '#7A3E63', '#F2A6C0'],
    tags: ['多巴胺', '莓粉', '回血'],
    lockScreenImage: { colors: ['#1E0F1A', '#5E2F4C'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#261420', '#6E3858'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#2B1626', '#7A3E63'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#7A3E63', '#F2A6C0'], start: [0, 0], end: [1, 1] },
    quotes: ['今天多巴胺满格', '快乐会传染', '我又活过来了'],
  },
  {
    id: 'dopamine-02',
    name: '落日回血',
    mood: 'dopamine',
    slogan: '把不开心晒干',
    description:
      '暖橘到莓紫的落日渐变，治愈系。适合散步、放空、慢慢把情绪晒回来的傍晚。',
    colors: ['#2A1A16', '#8A4A3A', '#F2B79A'],
    tags: ['落日', '治愈', '暖橘'],
    lockScreenImage: { colors: ['#1D110D', '#6A382C'], start: [0, 0], end: [1, 1] },
    homeScreenImage: { colors: ['#231512', '#7A4034'], start: [0, 0], end: [0, 1] },
    widgetImage: { colors: ['#2A1A16', '#8A4A3A'], start: [0, 0], end: [1, 0] },
    avatarImage: { colors: ['#8A4A3A', '#F2B79A'], start: [0, 0], end: [1, 1] },
    quotes: ['把坏心情晒干', '落日不会迟到', '慢慢回血'],
  },
];

// —— 数据访问 helper（把 mock 当假后端，未来换 API 只改这里）——

export const getAllThemes = (): Theme[] => THEMES;

export const getThemesByMood = (moodKey: string): Theme[] =>
  THEMES.filter((t) => t.mood === moodKey);

export const getThemeById = (id: string): Theme | undefined =>
  THEMES.find((t) => t.id === id);

/** 今日推荐：用日期做稳定的「伪随机」排序，保证当天结果一致 */
export const getDailyPicks = (count = 6): Theme[] => {
  const seed = new Date().getDate();
  return [...THEMES]
    .sort(
      (a, b) =>
        ((a.id.charCodeAt(0) + seed) % 7) - ((b.id.charCodeAt(0) + seed) % 7)
    )
    .slice(0, count);
};
