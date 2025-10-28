// 用户登录状态管理
import { UserApi } from '@/api';
import { Modal } from 'antd-mobile';
import { create } from 'zustand';

interface AuthState {
  token: string | null;
  user_id: number | null;
  username: string | null;
  avatar: string | null;

  setToken: (token: string) => void;
  setUser: () => void;
  clearAuth: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: null,
  user_id: null,
  username: null,
  avatar: null,

  setToken: (token) => set({ token }),
  setUser: async () => {
    try {
      const response = await UserApi.profile();
      set((state) => ({ ...state, user_id: response.id, username: response.username, avatar: response.avatar }));
    } catch (error) {
      console.error('获取用户信息失败:', error);
      Modal.show({
        content: '获取用户信息失败，请稍后重试!',
        closeOnMaskClick: true,
      });
    }
  },
  clearAuth: () => set({ token: null }),
}));
