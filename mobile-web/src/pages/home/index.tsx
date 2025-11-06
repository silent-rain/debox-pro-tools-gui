import { Card, Grid } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import {
  UserOutline,
  TeamOutline,
  MessageOutline,
  UserAddOutline,
  TruckOutline,
  HeartOutline,
} from 'antd-mobile-icons';
import { ROUTES } from '@/constants/routes';
import styles from './index.module.less';

const Home = () => {
  const navigate = useNavigate();

  const menuItems = [
    { title: '账号管理', path: ROUTES.ACCOUNT_MANAGEMENT, icon: <UserOutline /> },
    { title: '同步群组', path: ROUTES.SYNC_GROUP, icon: <TeamOutline /> },
    { title: '同步群员', path: ROUTES.SYNC_GROUP_MEMBER, icon: <UserAddOutline /> },
    { title: '批量聊天', path: ROUTES.CHAT, icon: <MessageOutline /> },
    { title: '一键拉群', path: ROUTES.CREATE_GROUP, icon: <TruckOutline /> },
    { title: '一键关注', path: ROUTES.FOLLOW_USER, icon: <HeartOutline /> },
  ];

  return (
    <div className={styles.container}>
      <h1 className={styles.title}>Debox 工具箱</h1>

      <Grid columns={2} gap={16}>
        {menuItems.map((item) => (
          <Grid.Item key={item.path}>
            <Card className={styles.menuCard} onClick={() => navigate(item.path)}>
              <div className={styles.cardContent}>
                <div className={styles.cardIcon}>{item.icon}</div>
                <div className={styles.cardTitle}>{item.title}</div>
              </div>
            </Card>
          </Grid.Item>
        ))}
      </Grid>
    </div>
  );
};

export default Home;
