// 路由封装
import { createBrowserRouter, RouterProvider, Routes } from 'react-router-dom';
import { Route } from 'react-router';
import { RouteConfig } from '@/typings/routes';
import { RootRoutes } from './routes';
import { AuthRoute } from './auth-route';
import { useAuthStore } from '@/stores';
export { RootRoutes } from './routes';

// 该实现已被替换, 代码可以考虑删除
export function Router2() {
  const router = createBrowserRouter(RootRoutes);

  return <RouterProvider router={router} />;
}

export default function Router() {
  const authStore = useAuthStore.getState();

  // 设置用户信息
  if (!authStore.user_id) {
    authStore.setUser();
  }

  const RouteAuthFun = (routeList: RouteConfig[], parentPath: string = ''): React.ReactNode => {
    return routeList.map((item) => {
      // 拼接当前路由的完整路径
      const currentPath = item.path || '';
      const fullPath = parentPath === '/' ? `/${currentPath}` : `${parentPath}/${currentPath}`.replace(/\/+/g, '/');

      return (
        <Route
          path={fullPath}
          element={
            <AuthRoute auth={item.meta?.auth} key={fullPath}>
              {item.element}
            </AuthRoute>
          }
          // element={item.element}
          key={fullPath}
        >
          {item.children && RouteAuthFun(item.children, fullPath)}
        </Route>
      );
    });
  };

  console.log(RouteAuthFun(RootRoutes));
  return <Routes>{RouteAuthFun(RootRoutes)}</Routes>;
}
// 拼接完整路径
// const fullPath = (path?: string, rootPath?: string) => {
//   if (!rootPath) {
//     return path;
//   }

//   if (rootPath == '/' && !path?.startsWith('/')) {
//     return `${rootPath}${path}`;
//   }
//   if (rootPath == '/' && path?.startsWith('/')) {
//     return `${path}`;
//   }

//   if (path?.startsWith('/')) {
//     return `${rootPath}${path}`;
//   }
//   return `${rootPath}/${path}`;
// };
