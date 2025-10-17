import { RouteObject } from '@/typings/routes';
import { RootRoutes } from '../routes';

const flattenedRoutesRecursion = (routes: RouteObject[]) => {
  const flattenedRoutes: RouteObject[] = [];
  for (const route of routes) {
    const rootPath = route.path;
    const children = route.children;
    if (!children) {
      flattenedRoutes.push(route);
      continue;
    }

    for (const route of children) {
      if (rootPath == '/' && !route.path?.startsWith('/')) {
        route.path = `${rootPath}${route.path}`;
      } else if (!route.path?.startsWith('/')) {
        route.path = `${rootPath}/${route.path}`;
      }
      flattenedRoutes.push(route);
      if (!route.children) {
        continue;
      }
      const routes = flattenedRoutesRecursion(route.children);
      routes.forEach((route) => {
        flattenedRoutes.push(route);
      });
    }
  }

  return flattenedRoutes;
};

export const flattenedRoutes = () => {
  const routes = RootRoutes();

  const flattenedRoutes = flattenedRoutesRecursion(routes);
  return flattenedRoutes;
};
