// 路由封装
import { createBrowserRouter, RouterProvider, Routes } from 'react-router-dom';
import { Route } from 'react-router';
import { RouteConfig } from '@/typings/routes';
import { RootRoutes } from './routes';
import { AuthRoute } from './auth-route';
export { RootRoutes } from './routes';

// 该实现已被替换, 代码可以考虑删除
export function Router2() {
  const router = createBrowserRouter(RootRoutes);

  return <RouterProvider router={router} />;
}

export default function Router() {
  const RouteAuthFun = (routeList: RouteConfig[]): React.ReactNode => {
    return routeList.map((item) => (
      <Route
        path={item.path}
        element={
          <AuthRoute auth={item.meta?.auth} key={item.path}>
            {item.element}
          </AuthRoute>
        }
        key={item.path}
      >
        {item.children && RouteAuthFun(item.children)}
      </Route>
    ));
  };

  return <Routes>{RouteAuthFun(RootRoutes)}</Routes>;
}
