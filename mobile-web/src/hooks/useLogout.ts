import { useCallback } from 'react';
import { useNavigate } from 'react-router';
import { ROUTES } from '@/constants/routes';
import { cacheTokenKey } from '@/constants/auth';

/**
 * 自定义 Hook：处理登出逻辑
 * @returns 返回包含登出函数的对象
 */
export function useLogout() {
  const navigate = useNavigate();

  const handleLogout = useCallback(() => {
    // 清除登录信息
    localStorage.removeItem(cacheTokenKey);
    navigate(ROUTES.LOGIN, { replace: true });
  }, [navigate]);

  return {
    logout: handleLogout,
  };
}
