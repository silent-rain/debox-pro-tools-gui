// 实现路由鉴权
import { useEffect } from 'react';
import { matchRoutes, useLocation, useNavigate } from 'react-router-dom';
import { Modal } from 'antd-mobile';
import { RootRoutes } from './routes';
import { cacheTokenKey } from '@/constant/auth';
import { useAuthStore } from '@/stores';
import { UserApi } from '@/api';

export const AuthRoute = ({ children, auth }: any) => {
  const navigate = useNavigate();
  const token = localStorage.getItem(cacheTokenKey) || '';
  const location = useLocation();
  const matches = matchRoutes(RootRoutes, location);
  const authStore = useAuthStore.getState();

  useEffect(() => {
    const fetchProfile = async () => {
      console.log('AuthRoute: ', token, auth);

      // 鉴权判断
      if (token === '' && auth) {
        Modal.show({
          content: '登录过期，请重新登录!',
          closeOnMaskClick: true,
        });

        navigate('/login');
        return;
      }

      // 用户信息判断
      if (!authStore.user_id) {
        const response = await UserApi.profile();
        authStore.setUser(response.id, response.username, response.avatar);
      }

      // 路由是否存在
      const isExist = matches?.some((item) => item.pathname === location.pathname);
      if (token && isExist) {
        if (location.pathname === '/') {
          navigate('/home');
        } else {
          navigate(location.pathname);
        }
      }
    };

    fetchProfile();
  }, [auth, authStore, location.pathname, matches, navigate, token]);

  return children;
};
