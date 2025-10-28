// 路由配置
import { Navigate } from 'react-router-dom';
import { lazy } from 'react';
import { RouteConfig } from '@/typings/routes';

const Login = lazy(() => import('@/pages/auth/login'));
const Register = lazy(() => import('@/pages/auth/register'));

const Layout = lazy(() => import('@/layouts'));
const Home = lazy(() => import('@/pages/home'));
const Todo = lazy(() => import('@/pages/todo'));
const Message = lazy(() => import('@/pages/message'));
const PersonalCenter = lazy(() => import('@/pages/personal-center'));

const AiPainting = lazy(() => import('@/pages/ai-painting'));
const AiPaintingTextToImageBase = lazy(() => import('@/pages/text-to-image-base'));
const About = lazy(() => import('@/pages/personal-center/about'));
const Help = lazy(() => import('@/pages/personal-center/help'));
const Logs = lazy(() => import('@/pages/personal-center/logs'));
const Settings = lazy(() => import('@/pages/personal-center/settings'));
const ImportAccount = lazy(() => import('@/pages/personal-center/import-account'));
const ImportGroup = lazy(() => import('@/pages/personal-center/import-group'));
const UserDetail = lazy(() => import('@/pages/personal-center/user-detail'));

// TabBar Routes
const tabBarRoutes: RouteConfig = {
  path: '/',
  element: <Layout />,
  children: [
    {
      path: 'home',
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
      path: 'personal-center',
      element: <PersonalCenter />,
      meta: {
        title: '我的',
        auth: true,
      },
    },
  ],
};

// 用户中心 Routes
const PersonalCenterRoutes: RouteConfig = {
  path: '/personal-center',
  element: <Layout />,
  children: [
    {
      path: 'user-detail',
      element: <UserDetail />,
      meta: {
        title: '用户详情',
      },
    },
    {
      path: 'import-account',
      element: <ImportAccount />,
      meta: {
        title: '导入账号',
      },
    },
    {
      path: 'import-group',
      element: <ImportGroup />,
      meta: {
        title: '导入群组',
      },
    },
    {
      path: 'settings',
      element: <Settings />,
      meta: {
        title: '设置',
      },
    },
    {
      path: 'logs',
      element: <Logs />,
      meta: {
        title: '日志',
      },
    },
    {
      path: 'help',
      element: <Help />,
      meta: {
        title: '帮助',
      },
    },
    {
      path: 'about',
      element: <About />,
      meta: {
        title: '关于',
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
  tabBarRoutes, // TabBar Routes
  PersonalCenterRoutes,
  aiPaintingRoutes,
  // {
  //   path: '/',
  //   element: <Navigate to='/login' />,
  // },
  {
    path: '*',
    element: <Navigate to='/404' />,
  },
];
