import { FC } from 'react';
import { useLocation, useNavigate } from 'react-router-dom';
import { TabBar } from 'antd-mobile';
import { tabBarRoutes } from '@/routes/routes';
import styles from './footer.module.less';

// 底部导航栏
const Footer: FC = () => {
  const navigate = useNavigate();
  const location = useLocation();
  const { pathname } = location;

  const setRouteActive = (value: string) => {
    navigate(value);
  };

  const tabs = tabBarRoutes.children!.map((item) => ({
    key: tabBarRoutes.path! + item.path,
    title: item.meta?.title,
    icon: item.meta?.icon,
  }));

  return (
    <TabBar
      className={styles.footer}
      activeKey={pathname}
      onChange={(value) => {
        setRouteActive(value);
      }}
    >
      {tabs.map((item) => (
        <TabBar.Item key={item.key} icon={item.icon} title={item.title} />
      ))}
    </TabBar>
  );
};

export default Footer;
