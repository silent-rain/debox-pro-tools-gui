// 路由

export interface RouteMeta {
  title: string;
  keepAlive?: boolean;
  auth?: boolean;
  key?: string;
}

export interface RouteConfig {
  path?: string;
  element?: React.ReactNode;
  children?: RouteObject[];
  redirect?: string;
  caseSensitive?: boolean;
  index?: false;
  meta?: RouteMeta;
  isLink?: string;
}
