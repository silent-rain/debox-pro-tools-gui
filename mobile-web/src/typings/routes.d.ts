// 路由

export interface RouteMeta {
  keepAlive?: boolean;
  requiresAuth?: boolean;
  title: string;
  key?: string;
}

export interface RouteObject {
  caseSensitive?: boolean;
  children?: RouteObject[];
  element?: React.ReactNode;
  index?: false;
  path?: string;
  meta?: RouteMeta;
  isLink?: string;
}
