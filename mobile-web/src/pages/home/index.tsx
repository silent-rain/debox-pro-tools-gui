import { Card, Grid } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import {
  UserOutline,
  TeamOutline,
  MessageOutline,
  UserAddOutline,
  TruckOutline,
  HeartOutline,
  LoopOutline,
} from 'antd-mobile-icons';
import { ROUTES } from '@/constants/routes';
import styles from './index.module.less';

const Home = () => {
  const navigate = useNavigate();

  const menuItems = [
    {
      title: '账号管理',
      path: ROUTES.ACCOUNT_MANAGEMENT,
      icon: <UserOutline />,
      description: '管理DeBox账号',
    },
    { title: '同步群组', path: ROUTES.SYNC_GROUP, icon: <TeamOutline />, description: '同步账号的群组' },
    {
      title: '同步群员',
      path: ROUTES.SYNC_GROUP_MEMBER,
      icon: <UserAddOutline />,
      description: '同步群组成员列表',
    },
    { title: '批量聊天', path: ROUTES.CHAT, icon: <MessageOutline />, description: '批量发送消息' },
    { title: '一键拉群', path: ROUTES.CREATE_GROUP, icon: <TruckOutline />, description: '快速创建群组' },
    {
      title: '一键关注',
      path: ROUTES.FOLLOW_USER,
      icon: <HeartOutline />,
      description: '一键关注目标账号',
    },
    { title: '自动互关', path: ROUTES.AUTO_FOLLOW, icon: <LoopOutline />, description: '账号之间的互相关注' }, // 群组互关/账号互关
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
                <div className={styles.cardDescription}>{item.description}</div>
              </div>
            </Card>
          </Grid.Item>
        ))}
      </Grid>
    </div>
  );
};

export default Home;
