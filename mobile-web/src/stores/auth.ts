// 用户登录状态管理
import { create } from 'zustand';

interface AuthState {
  token: string | null;
  user_id: number | null;
  username: string | null;
  avatar: string | null;

  setToken: (token: string) => void;
  setUser: (user_id: number, username: string, avatar: string) => void;
  clearAuth: () => void;
}

export const useAuthStore = create<AuthState>((set) => ({
  token: null,
  user_id: null,
  username: null,
  avatar: null,

  setToken: (token) => set({ token }),
  setUser: (user_id, username, avatar) => set({ user_id, username, avatar }),
  clearAuth: () => set({ token: null }),
}));
