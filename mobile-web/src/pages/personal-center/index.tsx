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
import styles from './index.module.less';

export const PersonalCenter = () => {
  const navigate = useNavigate();

  const features = [
    { title: '导入账号', path: '/import-account', icon: <UserAddOutline /> },
    { title: '导入群组', path: '/import-group', icon: <TeamOutline /> },
    { title: '设置', path: '/settings', icon: <SetOutline /> },
    { title: '日志', path: '/logs', icon: <FileOutline /> },
    { title: '帮助', path: '/help', icon: <QuestionCircleOutline /> },
    { title: '关于', path: '/about', icon: <InformationCircleOutline /> },
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

  const [userInfo] = useState({
    avatar: 'https://example.com/avatar.jpg',
    name: '用户名',
    bio: '欢迎来到我的个人中心！',
  });

  return (
    <div className={styles.userInfo}>
      <Avatar src={userInfo.avatar} style={{ '--size': '80px' }} onClick={() => navigate('/user-detail')} />
      <div className={styles.userText} onClick={() => navigate('/user-detail')}>
        <h3>{userInfo.name}</h3>
        <p>{userInfo.bio}</p>
      </div>
    </div>
  );
};

export default PersonalCenter;
