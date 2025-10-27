// 实现路由鉴权
import { useEffect } from 'react';
import { matchRoutes, useLocation, useNavigate } from 'react-router-dom';
import { Modal } from 'antd-mobile';
import { RootRoutes } from './routes';
import { cacheTokenKey } from '@/constant/auth';

export const AuthRoute = ({ children, auth }: any) => {
  const navigate = useNavigate();
  const token = localStorage.getItem(cacheTokenKey) || '';
  const location = useLocation();
  const matches = matchRoutes(RootRoutes, location);

  const isExist = matches?.some((item) => item.pathname === location.pathname);

  useEffect(() => {
    console.log(token, auth);
    if (token === '' && auth) {
      Modal.show({
        content: '登录过期，请重新登录!',
        closeOnMaskClick: true,
      });

      navigate('/login');
    } else if (token && isExist) {
      if (location.pathname === '/') {
        navigate('/home');
      } else {
        navigate(location.pathname);
      }
    }
  }, [token, location.pathname, auth, isExist, navigate]);

  return children;
};
