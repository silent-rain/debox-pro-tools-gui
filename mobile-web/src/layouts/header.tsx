import { NavBar } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import styles from './header.module.less';

interface HeaderProps {
  title: string;
}

// 头部导航栏
const Header = ({ title }: HeaderProps) => {
  const navigate = useNavigate();

  // 首页不显示返回按钮
  if (title === '首页') {
    return null;
  }

  return (
    <NavBar
      className={styles.header}
      backIcon={false}
      onBack={() => {
        navigate(-1);
      }}
    >
      {title}
    </NavBar>
  );
};

export default Header;
