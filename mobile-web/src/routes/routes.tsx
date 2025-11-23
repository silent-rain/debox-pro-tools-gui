// 路由配置
import { Navigate } from 'react-router-dom';
import { lazy } from 'react';
import { RouteConfig } from '@/typings/routes';
import { AppOutline, MessageOutline, UnorderedListOutline, UserOutline } from 'antd-mobile-icons';

const Login = lazy(() => import('@/pages/auth/login'));
const Register = lazy(() => import('@/pages/auth/register'));
const Error404 = lazy(() => import('@/pages/error404'));

// TabBar
const Layout = lazy(() => import('@/layouts'));
const Home = lazy(() => import('@/pages/home'));
const Todo = lazy(() => import('@/pages/todo'));
const Message = lazy(() => import('@/pages/message'));
const PersonalCenter = lazy(() => import('@/pages/personal-center'));

// 用户中心
const About = lazy(() => import('@/pages/personal-center/about'));
const Help = lazy(() => import('@/pages/personal-center/help'));
const Logs = lazy(() => import('@/pages/personal-center/logs'));
const Settings = lazy(() => import('@/pages/personal-center/settings'));
const ImportGroup = lazy(() => import('@/pages/personal-center/import-group'));
const UserDetail = lazy(() => import('@/pages/personal-center/user-detail'));

// 账号管理
const AccountManagement = lazy(() => import('@/pages/account-management'));
const AccountManagementForm = lazy(() => import('@/pages/account-management/form'));

// 同步群组
const SyncGroup = lazy(() => import('@/pages/sync-group'));

// 同步群员
const SyncGroupMember = lazy(() => import('@/pages/sync-group-member'));

// 账号互助
const FollowMutualInvite = lazy(() => import('@/pages/follow-mutual-invite'));

// 关注管理
const AccountFollowManagement = lazy(() => import('@/pages/account-follow-management'));

// TabBar Routes
export const tabBarRoutes: RouteConfig = {
  path: '/',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: 'home',
      element: <Home />,
      meta: {
        title: '首页',
        icon: <AppOutline />,
        auth: true,
      },
    },
    {
      path: 'create-group',
      element: <Todo />,
      meta: {
        title: '拉群',
        icon: <UnorderedListOutline />,
        auth: true,
      },
    },
    {
      path: 'chat',
      element: <Message />,
      meta: {
        title: '聊天',
        icon: <MessageOutline />,
        auth: true,
      },
    },
    {
      path: 'personal-center',
      element: <PersonalCenter />,
      meta: {
        title: '我的',
        icon: <UserOutline />,
        auth: true,
      },
    },
  ],
};

// 用户中心 Routes
const PersonalCenterRoutes: RouteConfig = {
  path: '/personal-center',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: 'user-detail',
      element: <UserDetail />,
      meta: {
        title: '用户详情',
        auth: true,
      },
    },
    {
      path: 'import-group',
      element: <ImportGroup />,
      meta: {
        title: '导入群组',
        auth: true,
      },
    },
    {
      path: 'settings',
      element: <Settings />,
      meta: {
        title: '设置',
        auth: true,
      },
    },
    {
      path: 'logs',
      element: <Logs />,
      meta: {
        title: '日志',
        auth: true,
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

// 账号管理 Routes
const AccountManagementRoutes: RouteConfig = {
  path: '/account-management',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: '',
      element: <AccountManagement />,
      meta: {
        title: '导入账号',
        auth: true,
      },
    },
    {
      path: 'form',
      element: <AccountManagementForm />,
      meta: {
        title: '账号表单',
        auth: true,
      },
    },
  ],
};

// 同步群组 Routes
const SyncGroupRoutes: RouteConfig = {
  path: '/sync-group',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: '',
      element: <SyncGroup />,
      meta: {
        title: '同步群组',
        auth: true,
      },
    },
  ],
};

// 同步群员 Routes
const SyncGroupMemberRoutes: RouteConfig = {
  path: '/sync-group-member',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: '',
      element: <SyncGroupMember />,
      meta: {
        title: '同步群组成员',
        auth: true,
      },
    },
  ],
};

// 账号互助 Routes
const FollowMutualInviteRoutes: RouteConfig = {
  path: '/follow-mutual-invite',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: '',
      element: <FollowMutualInvite />,
      meta: {
        title: '账号互助',
        auth: true,
      },
    },
  ],
};

// 关注管理 Routes
const AccountFollowManagementRoutes: RouteConfig = {
  path: '/account-follow-management',
  element: <Layout />,
  isLayout: true,
  children: [
    {
      path: '',
      element: <AccountFollowManagement />,
      meta: {
        title: '关注管理',
        auth: true,
      },
    },
  ],
};

// 主路由
export const RootRoutes: RouteConfig[] = [
  // {
  //   path: '',
  //   element: <Navigate to='/home' />,
  // },
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
    },
  },
  tabBarRoutes, // TabBar
  PersonalCenterRoutes, // 用户中心
  AccountManagementRoutes, // 账号管理
  SyncGroupRoutes, // 同步群组
  SyncGroupMemberRoutes, // 同步群员
  FollowMutualInviteRoutes, // 账号互助
  AccountFollowManagementRoutes, // 账号关注管理
  // {
  //   path: '/',
  //   element: <Navigate to='/login' />,
  // },
  {
    path: '/404',
    element: <Error404 />,
    meta: {
      title: '404',
      key: 'error404',
    },
  },
  {
    path: '*',
    element: <Navigate to='/404' />,
  },
];
