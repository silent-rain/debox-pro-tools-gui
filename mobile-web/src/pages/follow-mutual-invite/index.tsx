import { Card, Grid, Toast } from 'antd-mobile';
import { TeamOutline, HeartOutline, UndoOutline } from 'antd-mobile-icons';
import styles from './index.module.scss';
import { DeboxAccountApi } from '@/api';

enum Mode {
  AUTO = 'auto',
  FOLLOW = 'follow',
  GROUP = 'group',
}

// 账号相互关注
const followAccounts = async () => {
  await DeboxAccountApi.followAccounts();
};

// 账号之间的群组相互拉群
const crossAccountGroupInvite = async () => {
  await DeboxAccountApi.crossAccountGroupInvite();
};

const Home = () => {
  const menuItems = [
    {
      title: '自动互助',
      mode: Mode.AUTO,
      icon: <UndoOutline />,
      description: '自动互注/拉群',
    },
    { title: '相互关注', mode: Mode.FOLLOW, icon: <HeartOutline />, description: '账号相互关注' },
    {
      title: '相互拉群',
      mode: Mode.GROUP,
      icon: <TeamOutline />,
      description: '账号所有群组互拉',
    },
  ];

  const handleClick = async (mode: Mode) => {
    switch (mode) {
      case Mode.AUTO:
        await followAccounts();
        await crossAccountGroupInvite();
        Toast.show({
          content: '操作成功',
          position: 'top',
        });
        break;
      case Mode.FOLLOW:
        await followAccounts();
        Toast.show({
          content: '操作成功',
          position: 'top',
        });
        break;
      case Mode.GROUP:
        await crossAccountGroupInvite();
        Toast.show({
          content: '操作成功',
          position: 'top',
        });
        break;
      default:
        console.log('未知模式');
        break;
    }
  };

  return (
    <div className={styles.container}>
      <h1 className={styles.title}>互助工具箱</h1>

      <Grid columns={2} gap={16}>
        {menuItems.map((item) => (
          <Grid.Item key={item.mode}>
            <Card className={styles.menuCard} onClick={() => handleClick(item.mode)}>
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
