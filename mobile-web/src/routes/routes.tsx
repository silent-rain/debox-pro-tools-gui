import { Navigate } from 'react-router-dom';
import { lazy } from 'react';
import { RouteObject } from '@/typings/routes';

const Layout = lazy(() => import('@/layouts'));
const Home = lazy(() => import('@/pages/home'));
const Todo = lazy(() => import('@/pages/todo'));
const Message = lazy(() => import('@/pages/message'));
const PersonalCenter = lazy(() => import('@/pages/personal-center'));
const AiPainting = lazy(() => import('@/pages/ai-painting'));
const AiPaintingTextToImageBase = lazy(() => import('@/pages/text-to-image-base'));
const Login = lazy(() => import('@/pages/login'));
const Register = lazy(() => import('@/pages/register'));

// TabBar Routes
const tabBarRoutes: RouteObject = {
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
const aiPaintingRoutes: RouteObject = {
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

export const RootRoutes = (): RouteObject[] => [
  // TabBar Routes
  tabBarRoutes,
  {
    path: '/login',
    element: <Login />,
    meta: {
      requiresAuth: false,
      title: '登录',
      key: 'login',
    },
  },
  {
    path: '/register',
    element: <Register />,
    meta: {
      requiresAuth: false,
      title: '用户注册',
      key: 'register',
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
