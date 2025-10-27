// 用户登录状态管理
import { create } from 'zustand';

interface AuthState {
  token: string | null;
  user_id: string | null;
  username: string | null;

  setAuthData: (token: string, user_id: string, username: string) => void;
  clearAuthData: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: null,
  user_id: null,
  username: null,

  setAuthData: (token, user_id, username) => set({ token, user_id, username }),
  clearAuthData: () => set({ token: null, user_id: null, username: null }),
}));
