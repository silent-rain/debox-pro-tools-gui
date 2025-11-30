// 用户登录状态管理
import { UserApi } from '@/api';
import { create } from 'zustand';

interface AuthState {
  user_id: number | null;
  username: string | null;
  avatar: string | null;

  setUser: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: null,
  user_id: null,
  username: null,
  avatar: null,

  setUser: async () => {
    try {
      const response = await UserApi.profile();
      set((state) => ({ ...state, user_id: response.id, username: response.username, avatar: response.avatar }));
    } catch (error) {
      console.error('获取用户信息失败:', error);
      // Modal.show({
      //   content: '获取用户信息失败，请稍后重试!',
      //   closeOnMaskClick: true,
      // });
    }
  },
}));
