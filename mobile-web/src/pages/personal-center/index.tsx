import { useState } from 'react';
import { useNavigate } from 'react-router-dom';
import { Avatar, List } from 'antd-mobile';
import {
  FileOutline,
  InformationCircleOutline,
  QuestionCircleOutline,
  SetOutline,
  TeamOutline,
  UserAddOutline,
} from 'antd-mobile-icons';
import { useAuthStore } from '@/stores';
import { ROUTES } from '@/constants/routes';
import styles from './index.module.less';

export const PersonalCenter = () => {
  const navigate = useNavigate();

  const features = [
    { title: '导入账号', path: ROUTES.PERSONAL_CENTER_IMPORT_ACCOUNT, icon: <UserAddOutline /> },
    { title: '导入群组', path: ROUTES.PERSONAL_CENTER_IMPORT_GROUP, icon: <TeamOutline /> },
    { title: '设置', path: ROUTES.PERSONAL_CENTER_SETTINGS, icon: <SetOutline /> },
    { title: '日志', path: ROUTES.PERSONAL_CENTER_LOGS, icon: <FileOutline /> },
    { title: '帮助', path: ROUTES.PERSONAL_CENTER_HELP, icon: <QuestionCircleOutline /> },
    { title: '关于', path: ROUTES.PERSONAL_CENTER_ABOUT, icon: <InformationCircleOutline /> },
  ];

  return (
    <div className='personal-center'>
      <UserInfo />

      <List className={styles.features}>
        {features.map((item) => (
          <List.Item key={item.path} prefix={item.icon} onClick={() => navigate(item.path)}>
            {item.title}
          </List.Item>
        ))}
      </List>
    </div>
  );
};

// 用户信息
const UserInfo = () => {
  const navigate = useNavigate();
  const { username, avatar } = useAuthStore();

  const [userInfo] = useState({
    avatar: avatar ? avatar : 'https://example.com/avatar.jpg',
    username: username ? username : '未登录',
    bio: '欢迎来到我的个人中心！',
  });

  return (
    <div className={styles.userInfo}>
      <Avatar
        src={userInfo.avatar}
        style={{ '--size': '80px' }}
        onClick={() => navigate(ROUTES.PERSONAL_CENTER_USER_DETAIL)}
      />
      <div className={styles.userText} onClick={() => navigate(ROUTES.PERSONAL_CENTER_USER_DETAIL)}>
        <h3>{userInfo.username}</h3>
        <p>{userInfo.bio}</p>
      </div>
    </div>
  );
};

export default PersonalCenter;
