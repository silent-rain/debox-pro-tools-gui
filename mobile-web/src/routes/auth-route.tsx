// 实现路由鉴权
import { useEffect } from 'react';
import { matchRoutes, useLocation, useNavigate } from 'react-router-dom';
import { Modal } from 'antd-mobile';
import { RootRoutes } from './routes';
import { cacheTokenKey } from '@/constant/auth';
import { useAuthStore } from '@/stores';

export const AuthRoute = ({ children, auth }: any) => {
  const navigate = useNavigate();
  const token = localStorage.getItem(cacheTokenKey) || '';
  const location = useLocation();
  const matches = matchRoutes(RootRoutes, location);
  const authStore = useAuthStore.getState();

  const isRouteExist = matches?.some((item) => item.pathname === location.pathname);

  useEffect(() => {
    // 鉴权判断
    const handleAuthCheck = () => {
      if (token === '' && auth) {
        Modal.show({
          content: '登录过期，请重新登录!',
          closeOnMaskClick: true,
        });
        navigate('/login');
        return false;
      }
      return true;
    };

    // 路由是否存在
    const handleRouteNavigation = () => {
      if (token && isRouteExist) {
        if (location.pathname === '/') {
          navigate('/home');
        } else {
          navigate(location.pathname);
        }
      }
    };

    // 鉴权检查
    if (!handleAuthCheck()) {
      return;
    }

    // 路由跳转
    handleRouteNavigation();
  }, [token, auth, authStore.user_id, isRouteExist, location.pathname, navigate, authStore]);
  return children;
};
