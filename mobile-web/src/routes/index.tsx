// 路由封装
import { createBrowserRouter, RouterProvider } from 'react-router-dom';
import { RootRoutes } from './routes';
export { RouterGuard } from './router-guard';

// 该实现已被替换, 代码可以考虑删除
export const Router = () => {
  const router = createBrowserRouter(RootRoutes);

  return <RouterProvider router={router} />;
};
