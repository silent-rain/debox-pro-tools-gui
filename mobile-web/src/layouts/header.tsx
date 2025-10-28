import { NavBar } from 'antd-mobile';
import { useNavigate } from 'react-router-dom';
import styles from './header.module.less';

interface HeaderProps {
  title: string;
  path: string;
}

// 隐藏头部导航栏的返回按钮
const hideBackButton = (path: string) => {
  const titles = ['/', '/home', '/todo', '/message', '/personal-center'];
  if (titles.includes(path)) {
    return true;
  }
  return false;
};

// 头部导航栏
const Header = ({ title, path }: HeaderProps) => {
  const navigate = useNavigate();

  // 隐藏头部导航栏的返回按钮
  if (hideBackButton(path)) {
    return null;
  }

  return (
    <NavBar
      className={styles.header}
      onBack={() => {
        navigate(-1);
      }}
    >
      {title}
    </NavBar>
  );
};

export default Header;
