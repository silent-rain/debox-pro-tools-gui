import { RouteConfig } from '@/typings/routes';
import { RootRoutes } from '../routes';

const flattenedRoutesRecursion = (routes: RouteConfig[], parentPath: string = ''): RouteConfig[] => {
  return routes.reduce<RouteConfig[]>((acc, route) => {
    // 处理当前路由的路径
    const currentPath = route.path || '';
    const fullPath = parentPath === '/' ? `/${currentPath}` : `${parentPath}/${currentPath}`.replace(/\/+/g, '/');

    // 创建当前路由的副本，避免修改原始对象
    const currentRoute = { ...route, path: fullPath };

    // 如果没有子路由，直接添加到结果中
    if (!route.children) {
      return [...acc, currentRoute];
    }

    // 递归处理子路由
    const childRoutes = flattenedRoutesRecursion(route.children, fullPath);
    return [...acc, currentRoute, ...childRoutes];
  }, []);
};

export const flattenedRoutes = () => {
  const flattenedRoutes = flattenedRoutesRecursion(RootRoutes);
  return flattenedRoutes;
};
