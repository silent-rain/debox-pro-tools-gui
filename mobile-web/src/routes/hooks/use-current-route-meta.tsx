import { useLocation } from 'react-router-dom';
import { flattenedRoutes } from './use-flattened-routes';

/**
 * 返回当前路由Meta信息
 */
export function useCurrentRouteMeta() {
  const location = useLocation();
  const routes = flattenedRoutes();

  const { pathname } = location;

  const matchedRouteMeta = routes.find((item) => {
    return item.path === pathname;
  })?.meta;
  return matchedRouteMeta;
}
