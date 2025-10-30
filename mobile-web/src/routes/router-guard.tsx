// 路由守卫
import { useEffect } from 'react';
import { Route, Routes, useLocation, useNavigate } from 'react-router-dom';
import { Modal } from 'antd-mobile';
import { cacheTokenKey } from '@/constant/auth';
import { RouteConfig } from '@/typings/routes';
import { useAuthStore } from '@/stores';
import { ROUTES } from '@/constants/routes';
import { RootRoutes } from './routes';
import { flattenedRoutes } from './hooks/use-flattened-routes';

// 全局路由组件
export const RouterGuard = () => {
  const navigate = useNavigate();
  const location = useLocation();

  const authStore = useAuthStore();

  useEffect(() => {
    if (!authStore.user_id) {
      authStore.setUser();
    }
  }, [authStore]);

  useEffect(() => {
    // 获取扁平化路由
    const flatteRoutes = flattenedRoutes();
    // 获取当前路由
    const currentRoute = flatteRoutes.find((route) => route.path === location.pathname);

    // 全局导航守卫逻辑
    if (currentRoute?.meta?.auth && !isAuthenticated()) {
      Modal.show({
        content: '未登录，重定向到登录页',
        closeOnMaskClick: true,
      });
      navigate(ROUTES.LOGIN, { replace: true });
    }
  }, [location, navigate]); // 监听路由变化

  const authRouteNodes = (routeList: RouteConfig[], parentPath: string = ''): React.ReactNode => {
    return routeList.map((item) => {
      const currentPath = item.path || '';
      const fullPath = parentPath === '/' ? `/${currentPath}` : `${parentPath}/${currentPath}`.replace(/\/+/g, '/');

      return (
        <Route path={fullPath} element={item.element} key={fullPath}>
          {item.children && authRouteNodes(item.children, fullPath)}
        </Route>
      );
    });
  };

  return <Routes>{authRouteNodes(RootRoutes)}</Routes>;
};

// 是否已鉴权
const isAuthenticated = () => {
  return localStorage.getItem(cacheTokenKey) !== null;
};
