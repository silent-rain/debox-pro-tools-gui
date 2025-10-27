// 路由配置
import { Navigate } from 'react-router-dom';
import { lazy } from 'react';
import { RouteConfig } from '@/typings/routes';

const Layout = lazy(() => import('@/layouts'));
const Home = lazy(() => import('@/pages/home'));
const Todo = lazy(() => import('@/pages/todo'));
const Message = lazy(() => import('@/pages/message'));
const PersonalCenter = lazy(() => import('@/pages/personal-center'));
const AiPainting = lazy(() => import('@/pages/ai-painting'));
const AiPaintingTextToImageBase = lazy(() => import('@/pages/text-to-image-base'));
const Login = lazy(() => import('@/pages/auth/login'));
const Register = lazy(() => import('@/pages/auth/register'));

// TabBar Routes
const tabBarRoutes: RouteConfig = {
  path: '/',
  element: <Layout />,
  children: [
    {
      path: '',
      element: <Home />,
      meta: {
        title: '首页',
      },
    },
    {
      path: 'todo',
      element: <Todo />,
      meta: {
        title: '待办',
      },
    },
    {
      path: 'message',
      element: <Message />,
      meta: {
        title: '消息',
      },
    },
    {
      path: 'me',
      element: <PersonalCenter />,
      meta: {
        title: '我的',
      },
    },
  ],
};

// AI 绘图 Routes
const aiPaintingRoutes: RouteConfig = {
  path: '/ai-painting',
  element: <Layout />,
  children: [
    {
      path: '',
      element: <AiPainting />,
      meta: {
        title: 'AI绘图',
      },
    },
    {
      path: '/ai-painting/text-to-image-base',
      element: <AiPaintingTextToImageBase />,
      meta: {
        title: '文生图',
      },
    },
  ],
};

export const RootRoutes: RouteConfig[] = [
  // TabBar Routes
  tabBarRoutes,
  {
    path: '/login',
    element: <Login />,
    meta: {
      title: '登录',
      key: 'login',
    },
  },
  {
    path: '/register',
    element: <Register />,
    meta: {
      title: '用户注册',
      key: 'register',
      auth: true,
    },
  },
  // {
  //   path: '/',
  //   element: <Navigate to='/login' />,
  // },
  aiPaintingRoutes,
  {
    path: '*',
    element: <Navigate to='/404' />,
  },
];
